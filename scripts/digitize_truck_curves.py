#!/usr/bin/env python3
"""Digitise the HCM Chapter 25/26 truck-performance curves into src/hcm/common/truck_curves.rs.

HCM 7 publishes these curves as figures only. Neither chapter gives a closed form, and the
generating study (NCFRP Report 31) is not part of the manual, so the curves have to be measured
off the printed exhibits. This script does that measurement reproducibly, so that the numbers in
truck_curves.rs can be regenerated and audited rather than trusted.

Method
------
1. Rasterise the exhibit. Family A (spot rate) uses the image embedded in the chapter PDF at its
   native resolution; Family B (cumulative travel time) uses a 300 dpi page render, because one
   embedded raster (Exhibit 25-A15) is JPEG-degraded enough that its series cannot be separated
   by colour.
2. Calibrate the axes from the printed gridlines, fitting a single uniform spacing by least
   squares so that a missed or doubled gridline does not skew the scale.
3. Follow each series across the plot, matching its colour and requiring continuity in y. Dash
   gaps are bridged by extrapolating the local slope. Seeds are taken at a column where all nine
   series are separated, which also fixes their grade labels by rank.
4. Resample onto a 250 ft grid and emit Rust.

Validation
----------
The manual reads these same graphs by eye, and its two worked examples state twenty such reads.
`--check` reports every one of them against the digitised tables. The Rust module carries the
same twenty as tests, plus cross-exhibit checks that catch a mis-assigned series.

Usage
-----
    uv run scripts/digitize_truck_curves.py --check
    uv run scripts/digitize_truck_curves.py --emit > src/hcm/common/truck_curves.rs.data

Requires the HCM chapter PDFs at ../resource/chap25.pdf and ../resource/chap26.pdf relative to
the repository root, plus pdftoppm/pdfimages (poppler-utils), numpy and Pillow.
"""

import argparse
import subprocess
import sys
from pathlib import Path

try:
    import numpy as np
    from PIL import Image
except ImportError:  # pragma: no cover - the script is reproduction material, not a CI gate
    np = Image = None

RESOURCE = Path(__file__).resolve().parents[2] / "resource"
GRID = list(range(0, 10001, 250))
GRADES = (0, 2, 3, 5)



# ---- curve tracking ----
def load(path):
    return np.asarray(Image.open(path).convert('RGB')).astype(float)


def mask(a, sat=32, vmax=253):
    mx = a.max(2)
    mn = a.min(2)
    return (mx - mn > sat) & (mx < vmax)


def col_runs(m, a, x, maxlen=8):
    col = np.nonzero(m[:, x])[0]
    if len(col) == 0:
        return []
    grp = [[col[0]]]
    for v in col[1:]:
        if v - grp[-1][-1] <= 1:
            grp[-1].append(v)
        else:
            grp.append([v])
    out = []
    for g in grp:
        if len(g) > maxlen:
            continue
        sats = [a[y, x].max() - a[y, x].min() for y in g]
        y = g[int(np.argmax(sats))]
        out.append((float(np.mean(g)), a[y, x]))
    return out


def track(a, seed_x, seed_y, x0, x1, ref=None, ctol=60, ywin=14, maxgap=40):
    m = mask(a)
    if ref is None:
        rs = col_runs(m, a, seed_x)
        if not rs:
            raise RuntimeError('no run at seed column')
        ref = min(rs, key=lambda r: abs(r[0] - seed_y))[1]
    pts = {}

    def march(step):
        y = seed_y
        slope = 0.0
        gap = 0
        x = seed_x
        while True:
            x += step
            if x < x0 or x > x1:
                break
            pred = y + slope * step
            cands = [r for r in col_runs(m, a, x)
                     if np.linalg.norm(r[1] - ref) < ctol and abs(r[0] - pred) < ywin]
            if not cands:
                gap += 1
                if gap > maxgap:
                    break
                y = pred
                continue
            best = min(cands, key=lambda r: abs(r[0] - pred))
            ny = best[0]
            slope = 0.6 * slope + 0.4 * (ny - y) / (step * max(1, gap + 1))
            y = ny
            gap = 0
            pts[x] = y

    pts[seed_x] = seed_y
    march(1)
    march(-1)
    return ref, dict(sorted(pts.items()))


class Axes:
    def __init__(self, x_px, x_val, y_px, y_val):
        (self.xa, self.xb), (self.va, self.vb) = x_px, x_val
        (self.ya, self.yb), (self.wa, self.wb) = y_px, y_val

    def X(self, px):
        return self.va + (px - self.xa) * (self.vb - self.va) / (self.xb - self.xa)

    def Y(self, py):
        return self.wa + (py - self.ya) * (self.wb - self.wa) / (self.yb - self.ya)


def sample(pts, ax, at_vals, smooth=5):
    """Resample a tracked curve at given x data values (monotone in px)."""
    xs = np.array(sorted(pts))
    ys = np.array([pts[x] for x in xs], float)
    if smooth > 1 and len(ys) > smooth:
        k = np.ones(smooth) / smooth
        ys = np.convolve(np.pad(ys, (smooth // 2, smooth // 2), mode='edge'), k, 'valid')[:len(xs)]
    dx = ax.X(xs)
    dy = ax.Y(ys)
    return [float(np.interp(v, dx, dy)) for v in at_vals]

# ---- Family B helpers ----
def gridspacing(a):
    g=(np.abs(a[:,:,0]-a[:,:,1])<12)&(np.abs(a[:,:,1]-a[:,:,2])<12)&(a[:,:,0]<235)&(a[:,:,0]>110)
    def thin(v):
        o=[]
        for x in v:
            if not o or x-o[-1][-1]>6: o.append([x])
            else: o[-1].append(x)
        return [int(np.mean(q)) for q in o]
    cols=thin([i for i,c in enumerate(g.sum(0)) if c>a.shape[0]*0.40])
    rows=thin([i for i,c in enumerate(g.sum(1)) if c>a.shape[1]*0.45])
    def fit(p):
        p=np.array(p,float); d=np.diff(p)
        s0=float(np.median(d))
        n=np.round(d/s0)                      # how many grid steps each gap spans
        n=np.clip(n,1,None)
        s=float(np.sum(d)/np.sum(n))          # least-squares uniform spacing
        return s
    return fit(cols), fit(rows), cols, rows

def origin(a):
    m=mask(a)
    ys,xs=np.nonzero(m)
    x0=xs.min()
    sel=xs<x0+4
    return float(x0), float(ys[sel].mean())

def analyse(path, ymax, grades=(8,7,6,5,4,3,2,0,-5)):
    a=load(path)
    dx,dy,cols,rows=gridspacing(a)
    x0,y0=origin(a)
    ax=Axes((x0,x0+10*dx),(0,10000),(y0,y0-dy),(0,20))   # dy px per 20 s
    m=mask(a,sat=42)
    # seed column: as far right as possible with 9 clean runs
    seed=None
    for xt in range(int(x0+10*dx)-2, int(x0+5*dx), -1):
        rs=col_runs(m,a,xt)
        if len(rs)==9: seed=xt; break
    if seed is None:
        raise RuntimeError('no column with all 9 curves separated in %s' % path)
    xt=seed
    rs=sorted(rs, key=lambda r:r[0])   # top (small y) = biggest grade
    out={}
    for g,(sy,c) in zip(grades, rs):
        ref,pts=T.track(a,xt,sy,int(x0),int(x0+10*dx),ctol=30,ywin=10,maxgap=60)
        out[g]=(ref,pts)
    return ax,out,(x0,y0,dx,dy,xt,len(rs))

# ---- exhibit inventory ----

# Family A: spot travel time rate versus distance.
#   image, axis calibration (x px -> ft, y px -> s/mi), and one seed per series.
SPOT_EXHIBITS = {
    "25-20_SUT": dict(
        page=77, kind="embedded", index=0,
        axes=((75, 782), (0, 10000), (8, 522), (120, 40)),
        seeds={(0, "decel"): None, (0, "accel"): (200, 60.8),
               (2, "decel"): (700, 52.2), (2, "accel"): (700, 54.2),
               (3, "decel"): (700, 58.8), (3, "accel"): (300, 66.5),
               (5, "decel"): (700, 76.2), (5, "accel"): (300, 80.1)}),
    "25-21_TT": dict(
        page=78, kind="embedded", index=0,
        axes=((80, 779), (0, 10000), (7, 520), (180, 40)),
        seeds={(0, "decel"): None, (0, "accel"): (500, 52.1),
               (2, "decel"): (700, 59.6), (2, "accel"): (700, 64.7),
               (3, "decel"): (700, 71.5), (3, "accel"): (700, 76.3),
               (5, "decel"): (700, 105.8), (5, "accel"): (500, 106.7)}),
}

# Family B: cumulative travel time versus distance. Seeds are the travel time of each needed
# grade at the seed column, read off the column where all nine series separate. Identifying the
# series by rank alone is not safe: Exhibit 25-A15 loses its 7% series to an overlap at every
# candidate column, which silently shifts every label below it by one grade.
FAMB_EXHIBITS = {
    "25-22_SUT70":    dict(pdf="chap25", page=80,  seed=1839, T={0: 92.0, 2: 98.8, 3: 107.5, 5: 130.9}),
    "25-A6_SUT60":    dict(pdf="chap25", page=230, seed=1846, T={0: 93.6, 2: 104.0, 3: 113.3, 5: 137.7}),
    "25-A7_SUT65":    dict(pdf="chap25", page=231, seed=1830, T={0: 92.2, 2: 101.3, 3: 110.3, 5: 134.2}),
    "25-A15_TT50":    dict(pdf="chap25", page=239, seed=1770, T={0: 100.2, 2: 122.4, 3: 139.6, 5: 185.1}),
    "25-A16_TT55":    dict(pdf="chap25", page=240, seed=1861, T={0: 98.7, 2: 120.4, 3: 136.8, 5: 183.1}),
    "25-A18_TT65":    dict(pdf="chap25", page=242, seed=1847, T={0: 93.5, 2: 112.8, 3: 128.0, 5: 172.5}),
    "26-A4_SUTffs65": dict(pdf="chap26", page=172, seed=1831, T={0: 92.3, 2: 101.4, 3: 110.3, 5: 134.5}),
    "26-A9_TTffs65":  dict(pdf="chap26", page=177, seed=1847, T={0: 93.8, 2: 112.9, 3: 128.1, 5: 172.7}),
}

# The twenty reads the two worked examples state, with the page each appears on.
# kind: "rate" reads an ordinate (s/mi), "time" reads a cumulative travel time (s),
# "dist" reads an abscissa (ft).
PUBLISHED_READS = [
    (1,  "dist", "25-20", "SUT", 3, "decel", 55.4, 4100, "chap25 p.208"),
    (2,  "rate", "25-20", "SUT", 3, "decel", 10000, 59, "chap25 p.208"),
    (3,  "dist", "25-21", "TT",  3, "decel", 55.4, 2100, "chap25 p.208"),
    (4,  "rate", "25-21", "TT",  3, "decel", 10000, 73, "chap25 p.208"),
    (5,  "time", "25-A7_SUT65",  3, 7920, 87, "chap25 p.209"),
    (6,  "time", "25-A18_TT65",  3, 7920, 99, "chap25 p.209"),
    (7,  "dist", "25-20", "SUT", 2, "accel", 59.0, 4000, "chap25 p.212"),
    (8,  "rate", "25-20", "SUT", 2, "accel", 10000, 53, "chap25 p.212"),
    (9,  "dist", "25-21", "TT",  2, "accel", 73.0, 3360, "chap25 p.212"),
    (10, "rate", "25-21", "TT",  2, "accel", 10000, 63, "chap25 p.212"),
    (11, "time", "25-A6_SUT60",  2, 10000, 105, "chap25 p.213"),
    (12, "time", "25-A15_TT50",  2, 10000, 125, "chap25 p.213"),
    (13, "dist", "25-20", "SUT", 5, "decel", 55.4, 1500, "chap25 p.216"),
    (14, "rate", "25-20", "SUT", 5, "decel", 6780, 75, "chap25 p.216"),
    (15, "dist", "25-21", "TT",  5, "decel", 63.0, 2050, "chap25 p.216"),
    (16, "rate", "25-21", "TT",  5, "decel", 7330, 103, "chap25 p.216"),
    (17, "time", "25-A7_SUT65",  5, 5280, 67, "chap25 p.217"),
    (18, "time", "25-A16_TT55",  5, 5280, 89, "chap25 p.217"),
    (19, "time", "26-A4_SUTffs65", 5, 10000, 134, "chap26 p.72"),
    (20, "time", "26-A9_TTffs65",  5, 10000, 173, "chap26 p.72"),
]



# ---- rasterising ----

def raster_embedded(pdf, page, index, out_dir):
    """Extract the images embedded in one PDF page at their native resolution."""
    stem = out_dir / f"{pdf}_p{page}"
    subprocess.run(["pdfimages", "-png", "-f", str(page), "-l", str(page),
                    str(RESOURCE / f"{pdf}.pdf"), str(stem)], check=True)
    return sorted(out_dir.glob(f"{pdf}_p{page}-*.png"))[index]


def raster_page(pdf, page, out_dir, dpi=300):
    """Render a whole page and crop to its coloured plot area."""
    stem = out_dir / f"{pdf}_hi{page}"
    subprocess.run(["pdftoppm", "-f", str(page), "-l", str(page), "-r", str(dpi), "-png",
                    str(RESOURCE / f"{pdf}.pdf"), str(stem)], check=True)
    src = sorted(out_dir.glob(f"{pdf}_hi{page}-*.png"))[0]
    im = Image.open(src).convert("RGB")
    a = np.asarray(im).astype(int)
    ys, xs = np.nonzero((a.max(2) - a.min(2)) > 45)
    box = (max(0, xs.min() - 150), max(0, ys.min() - 70),
           min(im.width, xs.max() + 70), min(im.height, ys.max() + 100))
    crop = out_dir / f"{pdf}_crop{page}.png"
    im.crop(box).save(crop)
    return crop


# ---- digitising ----

def spot_tables(out_dir):
    """Family A: one table per class, grade and branch."""
    tables = {}
    for name, spec in SPOT_EXHIBITS.items():
        cls = name.split("_")[1]
        img = raster_embedded("chap25", spec["page"], spec["index"], out_dir)
        a = load(img)
        (xa, xb), (va, vb), (ya, yb), (wa, wb) = spec["axes"]
        ax = Axes((xa, xb), (va, vb), (ya, yb), (wa, wb))
        for (grade, branch), seed in spec["seeds"].items():
            if seed is None:
                # 0% decelerating: the truck holds 75 mi/h, drawn as a flat line at 48 s/mi.
                tables[(cls, grade, branch)] = [48.0] * len(GRID)
                continue
            sx, rate = seed
            py = ya + (rate - wa) * (yb - ya) / (wb - wa)
            _, pts = track(a, sx, py, int(xa) + 1, int(xb), ctol=25, ywin=9, maxgap=80)
            v = sample(pts, ax, GRID, smooth=7)
            if branch == "decel":
                v[0] = 48.0
            tables[(cls, grade, branch)] = [round(float(x), 2) for x in v]
    return tables


def famb_tables(out_dir):
    """Family B: one table per exhibit and grade."""
    tables = {}
    for name, spec in FAMB_EXHIBITS.items():
        img = raster_page(spec["pdf"], spec["page"], out_dir)
        a = load(img)
        dx, dy, _, _ = gridspacing(a)
        x0, y0 = origin(a)
        ax = Axes((x0, x0 + 10 * dx), (0, 10000), (y0, y0 - dy), (0, 20))
        tab = {}
        for grade, Tv in spec["T"].items():
            sy = y0 - Tv / 20.0 * dy
            _, pts = track(a, spec["seed"], sy, int(x0), int(x0 + 10 * dx),
                           ctol=35, ywin=8, maxgap=80)
            xs = np.array(sorted(pts), float)
            ys = np.array([pts[x] for x in xs], float)
            d = np.abs(np.diff(ys))
            first = int(np.argmax(d > 0.8)) if (d > 0.8).any() else 0
            xs, ys = xs[first:], ys[first:]
            # Anchor the origin: every curve starts at T = 0, and near it the nine series are
            # too close together to separate by colour.
            dxv = np.concatenate([[0.0], ax.X(xs)])
            dyv = np.concatenate([[0.0], ax.Y(ys)])
            k = np.ones(9) / 9
            dyv[1:] = np.convolve(np.pad(dyv[1:], (4, 4), mode="edge"), k, "valid")[:len(dyv) - 1]
            v = np.maximum.accumulate(np.interp(GRID, dxv, dyv))
            tab[grade] = [round(float(x), 2) for x in v]
        tables[name] = tab
    return tables


def at(table, x):
    return float(np.interp(x, GRID, table))


def inverse(table, rate):
    v = np.array(table)
    x = np.array(GRID, float)
    return float(np.interp(rate, v, x)) if v[-1] > v[0] else float(np.interp(-rate, -v, x))


def check(spot, famb):
    """Report each published read. Tolerance is the manual's own fidelity, about 1 s/mi."""
    print(f"{'#':>3}  {'read':<34} {'digitised':>10} {'published':>10} {'diff':>8}  result")
    worst = 0.0
    for row in PUBLISHED_READS:
        n, kind = row[0], row[1]
        if kind == "time":
            _, _, exhibit, grade, ft, book, page = row
            got = at(famb[exhibit][grade], ft)
            label, tol, unit = f"{exhibit} {grade}% @{ft} ft", 1.1, "s"
        elif kind == "rate":
            _, _, exh, cls, grade, branch, ft, book, page = row
            got = at(spot[(cls, grade, branch)], ft)
            label, tol, unit = f"{exh} {cls} {grade}% {branch} @{ft} ft", 1.3, "s/mi"
        else:
            _, _, exh, cls, grade, branch, rate, book, page = row
            got = inverse(spot[(cls, grade, branch)], rate)
            label, tol, unit = f"{exh} {cls} {grade}% {branch} x@{rate}", 250.0, "ft"
        d = got - book
        worst = max(worst, abs(d) / tol)
        print(f"{n:>3}  {label:<34} {got:>10.2f} {book:>10} {d:>+8.2f} {unit:<5} "
              f"{'HIT' if abs(d) <= tol else 'MISS'}  {page}")
    return 0 if worst <= 1.0 else 1


# ---- emitting ----

def fmt(vals, per_line=6):
    return "\n".join("    " + " ".join("%8.2f," % v for v in vals[i:i + per_line])
                      for i in range(0, len(vals), per_line))


def emit(spot, famb):
    out = []
    out.append("pub const STATIONS_FT: [f64; %d] = [" % len(GRID))
    out.append(fmt([float(g) for g in GRID]))
    out.append("];\n")
    for (cls, grade, branch), v in sorted(spot.items()):
        out.append("static SPOT_%s_%s_G%d: [f64; %d] = [" %
                   (cls, branch.upper(), grade, len(GRID)))
        out.append(fmt(v))
        out.append("];\n")
    for name, tab in FAMB_EXHIBITS.items():
        stem = FAMB_RUST_NAMES[name]
        for grade in GRADES:
            out.append("static %s_G%d: [f64; %d] = [" % (stem, grade, len(GRID)))
            out.append(fmt(famb[name][grade]))
            out.append("];\n")
    return "\n".join(out)


FAMB_RUST_NAMES = {
    "25-A6_SUT60": "FAMB_SUT_INIT60", "25-A7_SUT65": "FAMB_SUT_INIT65",
    "25-22_SUT70": "FAMB_SUT_INIT70", "25-A15_TT50": "FAMB_TT_INIT50",
    "25-A16_TT55": "FAMB_TT_INIT55", "25-A18_TT65": "FAMB_TT_INIT65",
    "26-A4_SUTffs65": "FAMB_SUT_FFS65", "26-A9_TTffs65": "FAMB_TT_FFS65",
}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--check", action="store_true",
                    help="report the twenty published reads against the digitised tables")
    ap.add_argument("--emit", action="store_true", help="write the Rust data tables to stdout")
    ap.add_argument("--work", default="/tmp/hcm_truck_curves", help="scratch directory")
    args = ap.parse_args()
    if not (args.check or args.emit):
        ap.error("choose --check or --emit")
    if np is None:
        print("numpy and Pillow are required: uv pip install numpy pillow", file=sys.stderr)
        return 2
    if not (RESOURCE / "chap25.pdf").exists():
        print(f"missing {RESOURCE / 'chap25.pdf'}; the HCM chapter PDFs are not in the repo",
              file=sys.stderr)
        return 2
    work = Path(args.work)
    work.mkdir(parents=True, exist_ok=True)
    spot = spot_tables(work)
    famb = famb_tables(work)
    rc = check(spot, famb) if args.check else 0
    if args.emit:
        print(emit(spot, famb))
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
