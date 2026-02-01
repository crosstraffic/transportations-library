# **CHAPTER 10** **FREEWAY FACILITIES CORE METHODOLOGY** **CONTENTS**

**1. INTRODUCTION**

Overview

Chapter Organization

Related HCM Content


**2. CONCEPTS**

Overview

Sections, Segments, and Influence Areas

Free-Flow Speed

Freeway Facility Capacity

LOS: Component Segments and the Freeway Facility


**3. METHODOLOGY**

Scope of the Methodology

Required Data and Sources

Overview

Computational Steps


**4. EXTENSIONS TO THE METHODOLOGY**

Work Zone Analysis


Managed Lanes Analysis

Active Traffic and Demand Management


**5. APPLICATIONS**

Example Problems

Related Content in the HCMAG

Example Results

Planning, Preliminary Engineering, and Design Analysis

Use of Alternative Tools


**6. REFERENCES**

# **LIST OF EXHIBITS**


Exhibit 10-1: Influence Areas of Merge, Diverge, and Weaving Segments
Without Managed Lanes

Exhibit 10-2: Sections and Segments on an Urban Freeway

Exhibit 10-3: Typology of Managed Lane Access Point Designs

Exhibit 10-4: Sample Facility with Five Sections

Exhibit 10-5: Example of the Effect of Capacity on Demand and Actual Flow
Rates on a Freeway Facility

Exhibit 10-6: LOS Criteria for Urban and Rural Freeway Facilities

Exhibit 10-7: Required Input Data, Potential Data Sources, and Default
Values for Freeway Facility Analysis

Exhibit 10-8: Freeway Facility Methodology

Exhibit 10-9: Example Freeway Facility for Time–Space Domain Illustration

Exhibit 10-10: Example Time–Space Domain for Freeway Facility Analysis

Exhibit 10-11: Defining Analysis Segments for a Ramp Configuration


Exhibit 10-12: Defining Analysis Segments for a Weaving Configuration

Exhibit 10-13: Node–Segment Representation of a Freeway Facility

Exhibit 10-14: Mainline and Segment Flow at On- and Off-Ramps

Exhibit 10-15: Lane Closure Severity Index Values for Different Lane
Closure Configurations

Exhibit 10-16: Example of Minimum Lateral Clearance in Work Zone

Exhibit 10-17: Freeway Work Zone with One Open Lane, Trucks, and a Long
Upgrade

Exhibit 10-18: Graphical Illustration of the Managed Lane Segmentation
Method

Exhibit 10-19: Cross-Weave Movement Associated with Managed Lane
Access and Egress

Exhibit 10-20: List of Example Problems

Exhibit 10-21: Facility Travel Time Sensitivity to Free-Flow Speed

Exhibit 10-22: Facility Travel Time Sensitivity to _d/c_ Ratio on Critical
Segment

Exhibit 10-23: Facility Travel Time Sensitivity to Percentage Drop in Queue
Discharge Rate

Exhibit 10-24: Limitations of the HCM Freeway Facilities Analysis
Procedure


# **1. INTRODUCTION**

**OVERVIEW**

This chapter provides the core methodology for analyzing extended
lengths of freeway composed of continuously connected basic freeway,
weaving, merge, and diverge segments. Such extended lengths are referred to
as a _freeway facility._ In this terminology, _facility_ does not refer to an entire
freeway from beginning to end; instead, it refers to a specific set of
connected segments that have been identified for analysis. In addition, the
term does not refer to a freeway system consisting of several interconnected
freeways.

This chapter’s methodology relies on the freeway segment methodologies
in Chapters 12, 13, and 14. These methods focus on a single analysis period
of interest, generally the peak 15 min within a peak hour. The methodology
allows for the analysis of multiple and contiguous 15-min analysis periods
and is capable of identifying breakdowns and the impact of such breakdowns
over space and time. In essence, the methodology amalgamates hundreds or
thousands of individual segment–analysis period analyses into a single
facility analysis. It also allows for managed lanes and work zone analysis.

The methodology also is the basis of both freeway reliability analysis
and the assessment of active traffic and demand management (ATDM)
strategies. Both of these applications are described in Chapter 11, Freeway
Reliability Analysis. Conceptually, Chapter 10 is a prerequisite for any
reliability or ATDM analysis.

This chapter discusses the basic principles of the methodology and their
application. Chapter 25, Freeway Facilities: Supplemental, provides a
detailed description of all the algorithms that define the methodology. The
methodology is integrated with the FREEVAL computational engine, which
implements the complex computations involved. Volume 4 contains a user’s
guide to FREEVAL and an executable, research-grade software engine that
implements the methodology.


**CHAPTER ORGANIZATION**

Section 2 presents the basic concepts of freeway facility operations,
including definitions of analysis segments, capacity and free-flow speed
concepts, and the level of service (LOS) framework for freeway facilities.

Section 3 presents the base methodology for evaluating freeway
facilities, including details on all computational steps in the evaluation of a
freeway facility.

Section 4 extends the core method presented in Section 3 to applications
for managed lanes, including high-occupancy vehicle (HOV) and highoccupancy toll (HOT) lanes under various types of separation from the
general purpose lanes. This method is based on the findings from National
Cooperative Highway Research Program (NCHRP) Project 03-96 ( _1–3_ ).
Additional extensions include adaptations of the method for the evaluation of
short-term and long-term work zones based on the findings from NCHRP
Project 03-107 ( _4, 5_ ).

Section 5 presents application guidance on using the results of a freeway
facility analysis, including example results from the methods, information on
the sensitivity of results to various inputs, and service volume tables.


**RELATED HCM CONTENT**

Other _Highway Capacity Manual_ (HCM) content related to this chapter
includes the following:

   - Chapter 3, Modal Characteristics, where the Variations in Demand
subsection for the automobile mode describes typical travel demand
patterns for freeway and multilane highway segments;

   - Chapter 4, Traffic Operations and Capacity Concepts, which
provides background for the capacity and breakdown definitions
specific to freeway and multilane highway segments that are
presented in this chapter’s Section 2;

   - Chapter 11, Freeway Reliability Analysis, which provides extensions
of the core freeway facility methodology for performing a whole-year
reliability analysis and for assessing the whole-year impacts of
ATDM strategies;


- Chapters 12, 13, and 14, which present the segment methodologies
for basic freeway and multilane highway segments, freeway weaving
segments, and freeway merge and diverge segments, respectively;

- Chapter 25, Freeway Facilities: Supplemental, which provides
additional details for this methodology, including a detailed
description of the oversaturated procedure, and a summary of the
computational engine for freeway facility analysis;

- Chapter 26, Freeway and Highway Segments: Supplemental, which
provides additional details for basic freeway segment capacity
measurement and driver population factors;

- Case Study 4, New York State Route 7, in the HCM Applications
Guide in Volume 4, which demonstrates how this chapter’s methods
can be applied to the evaluation of an actual freeway facility; and

- Section H, Freeway Analyses, in the _Planning and Preliminary_
_Engineering Applications Guide to the HCM,_ found in Volume 4,
which describes how to incorporate this chapter’s methods and
performance measures into a planning effort.


# **2. CONCEPTS**

**OVERVIEW**

A freeway is a separated highway with full control of access having two
or more lanes in each direction dedicated to the exclusive use of motorized
traffic. Freeway facilities are composed of various uniform segments that
may be analyzed to determine capacity and LOS. Three types of segments are
found on freeways:

   - _Basic freeway segments_ are all segments that are not merge, diverge,
or weaving segments(whether general purpose or managed lanes.
They are described in more detail in Chapter 12.

   - _Freeway weaving segments_ are segments in which two or more
traffic streams traveling in the same general direction cross paths
along a significant length of freeway without the aid of traffic control
devices (except for guide signs). Weaving segments are formed when
a diverge segment closely follows a merge segment or when a onelane off-ramp closely follows a one-lane on-ramp and the two are
connected by a continuous auxiliary lane. These segment types occur
on both general purpose and managed lane facilities. In the latter
case, and depending on the geometry, that segment could be labeled
as a managed lane (ML) weaving or an ML access segment. Details
for those designations are provided in Chapter 13.

   - _Freeway merge and diverge segments_ are segments in which two or
more traffic streams combine to form a single traffic stream (merge)
or a single traffic stream divides to form two or more separate traffic
streams (diverge). These segment types occur on both general
purpose and managed lane facilities. Details for those segments are
provided in Chapter 14.

This chapter covers the core freeway facilities methodology, which may
include managed lanes as part of the facility. The analysis is limited to a
single study period not to exceed 24 h. Extensions of the core method to


longer study periods are covered in Chapter 11, Freeway Reliability
Analysis. Those extensions are intended to account for ( _a_ ) the longer-term
effects of both recurring (i.e., bottlenecks) and nonrecurring (e.g., due to
weather, incidents, or work zones) congestion on freeway facility operations
and ( _b_ ) the effects of ATDM strategies in mitigating some of those negative
impacts.


**SECTIONS, SEGMENTS, AND INFLUENCE AREAS**


**Facilities Without Managed Lanes**

The definitions of freeway sections and freeway segments and their
respective influence areas should be clearly understood.

_Sections_ are defined as extending from ramp gore point to gore point and
are most directly compatible with the freeway performance databases used
by many agencies. Some of these databases further distinguish between
_internal sections_ (e.g., between an off-ramp and an on-ramp at a diamond
interchange) and _external sections_ (between the final on-ramp at one
interchange and the first off-ramp at the next downstream interchange). For
the purpose of the HCM methodology, the distinction between internal and
external sections is of no consequence. Sections are used in the planninglevel application of this method detailed in Section 4 as well as for
calibrating and validating freeway facilities, since sections are more directly
compatible with field data than are HCM segments.

_Segments_ are the portions of freeway sections corresponding to the
definitions in the analysis methodologies presented in Chapters 12, 13, and
14. Segments can be identified by considering the area where a merge,
diverge, or weave influences facility operations.

The _influence areas_ of merge, diverge, and weaving segments are as
follows:

   - For _weaving segments,_ the base length of the weaving segment plus
500 ft upstream of the entry point to the weaving segment and 500 ft
downstream of the exit point from the weaving segment; entry and exit
points are defined as the points where the appropriate edges of the
merging and diverging lanes meet;


   - For _merge segments,_ from the point where the edges of the travel
lanes of the merging roadways meet to a point 1,500 ft downstream of
that point; and

   - For _diverge segments,_ from the point where the edges of the travel
lanes of the diverging roadways meet to a point 1,500 ft upstream of
that point.

The influence areas of merge, diverge, and weaving segments are
illustrated in Exhibit 10-1. A weaving segment is usually defined as the
distance between the on-ramp and off-ramp gore points. However, its
influence area extends 500 ft upstream and downstream of the gore-to-gore
length as defined above.


**Exhibit 10-1: Influence Areas of Merge, Diverge, and Weaving Segments Without**
**Managed Lanes**


(a) Merge Influence Area


(b) Diverge Influence Area


(c) Weaving Influence Area


Basic freeway segments are any other segments along the freeway that
are not within these defined influence areas. This does not imply that basic
freeway segments are unaffected by the presence of nearby merge, diverge,
and weaving segments. For example, the effects of a breakdown in a merge
segment will propagate to both upstream and downstream segments,
regardless of type. The impact of the frequency of merge, diverge, and
weaving segments on the general operation of all segments is taken into
account by the free-flow speed of the facility.

Basic freeway segments, therefore, exist even on urban freeways where
merge and diverge points (most often ramps) are closely spaced. Exhibit 102 demonstrates this point by illustrating the definition of sections and
segments. It shows a 9,100-ft (1.7-mi) length of freeway with four ramp
terminals, two of which form a weaving segment. Overall, five sections are
divided into six segments for consistency with the definition of HCM
segments and their influence areas above.


**Exhibit 10-2: Sections and Segments on an Urban Freeway**


Even with an average ramp spacing of less than 0.5 mi, this length of
freeway contains three basic freeway segments. The lengths of these
segments are relatively short, but in terms of analysis methodologies they


must be treated as basic freeway segments. Thus, while many urban freeways
will be dominated by frequent merge, diverge, and weaving segments, there
will still be segments classified and analyzed as basic freeway segments.

In applying the freeway facility methodology, the practice of beginning
and ending the facility with a basic freeway segment is highly recommended.
This segment may contain a _partial section_, since it does not both begin and
end at a gore point. Sections 1 and 5 in Exhibit 10-2 are examples of partial
sections. In comparing HCM results with field data, the analyst should
consider that the length of the partial section will likely be less than the
length of the section used in the database the field data came from.

The core methodology requires that all queues be contained within the
facility. Thus, the first (basic) segment on the facility should be an upstream
location that queues do not reach. Similarly, the last downstream (basic)
segment should not be affected by queues spilling back from locations
downstream of the defined facility.

Second, segment boundaries do not necessarily coincide with section
boundaries. For example, the first weaving segment (Segment 2) in Exhibit
10-2 extends upstream and downstream of Section 2. The segment extends
beyond the gore points, consistent with the definition of the weaving
influence area in Chapter 13. Because field data are likely reported to cover
the extent of Section 2 (but not beyond), this difference represents a potential
source of error in the calibration and validation effort.

In addition, HCM sections are homogeneous, but actual freeway sections
are not necessarily homogeneous. For example, detectors are not necessarily
matched with section definitions, resulting in either missing data points or
locations with multiple sensors in one section. Guidance for field
measurement and detector location is given in Chapter 26.


**Facility Segmentation Guidance**

Facility segmentation is only a small part of the overall freeway facility
methodology, but it is highly important in ensuring the proper application of
the methodology. Segmentation usually requires significant analyst time to
ensure that the segment types and the computational procedures for those
segments have been correctly entered. The segmentation step must be carried
out by the analyst before any computations are performed.


This section provides guidance on segmenting facilities. However, the
wide variety of configurations and conditions found on freeway facilities
means that engineering judgment, beyond the guidelines specified here, may
need to be applied in certain cases.

There are two basic steps in defining a freeway facility:

   - Deciding appropriate facility termini and overall length, and

   - Dividing the facility into HCM segments for analysis purposes.

For the first step, facility termini should be based on the following
locations, which are provided in rank order ( _6_ ):

1. Major freeway-to-freeway system interchanges;

2. Nonadjacent urbanized area boundaries;

3. Major intersecting (nonfreeway) routes;

4. Other special considerations such as major traffic generators (e.g.,
central city downtowns, airports) or state boundaries; and

5. Length, with consideration given to the type of area where the
freeway is located, as well as the maximum facility length discussed
in Section 3.

The rules above represent general guidance, but facilities may need to be
extended or shortened to serve the purpose of the analysis. For example, as
noted above, it is recommended that queues be contained within the defined
facility if at all possible. When multiple consecutive facilities are analyzed,
they should not overlap. A total facility length that is less than or equal to the
distance that can be traversed within a 15-min analysis period at the freeflow speed is also recommended. If necessary, a longer study section can be
subdivided into multiple smaller facilities for analysis.

The following general segmentation rules apply for the second step,
dividing a facility into HCM segments:

   - The first and last segments of the defined facility are recommended to
be basic freeway segments.

   - A new segment should be started whenever demand volume changes
(i.e., at on- and off-ramps and at at-grade access points to managed


lanes).

- A new segment should be started whenever capacity changes (i.e.,
when a full or auxiliary lane is added, when one or more lanes are
added or dropped, when the terrain changes significantly, or where
lane widths or lateral clearances change in a way that affects
capacity).

- The influence area of a ramp is considered to be 1,500 ft, measured
downstream from the gore point for on-ramps and upstream of the
gore point for off-ramps. The end of a merge segment’s ramp
influence area often represents a transition to a basic freeway
segment. Similarly, a basic segment transitions to a diverge segment
at the beginning of the ramp influence area.

- Ramp segments, including the ramp influence area, are classified
either as merge (on-ramp) or as diverge (off-ramp) segments, unless
two adjacent merge and diverge segments are connected by an
auxiliary lane, in which case the entire segment is coded as a
weaving segment. In the latter case, the weave influence area extends
500 ft upstream and 500 ft downstream of the two respective gore
areas (see Exhibit 10-2).

- When the gore-to-gore length between two adjacent merge and
diverge segments exceeds 3,000 ft and no auxiliary lane exists, the
section should be coded as a series of three segments (merge, basic,
diverge). The basic segment length is the difference between the
gore-to-gore spacing and 3,000 ft.

- When the gore-to-gore length of two adjacent merge and diverge
segments is less than 3,000 ft but longer than 1,500 ft and no auxiliary
lane exists, the section should be coded as a series of three segments,
with the middle segment being defined as an overlap segment (merge,
overlap, diverge). In this case, the overlap segment length is the
difference between 3,000 ft and the gore-to-gore spacing, and the
merge and diverge segment lengths are equal to the gore-to-gore
spacing minus 1,500 ft.

- It is highly unusual to have ramp spacing (combinations of merge and
diverge) less than 1,500 ft without the addition of an auxiliary lane to


connect the two gore areas. However, when this occurs, the 1,500-ft
merge or diverge segment length is truncated at the adjacent ramp
gore point.

   - Any remaining unassigned segments after all merge, diverge, weave,
and overlap segments have been defined are labeled as basic
segments.


**Facilities with Managed Lanes**

When managed lanes are present, additional managed lane segment types
are defined, as explained in Chapters 12–14. Extensions to the methodology
to address managed lanes are presented in Section 4 of this chapter.

Two important concepts related to managed lane segmentation are of
interest:

   - The _lane group concept_, where each managed lane segment must be
paired with an adjacent general purpose segment having the same
length. This concept is explained in more detail in Section 4.

   - The _friction_ and _cross-weave concepts_, which describe how the
general purpose and managed lane segments affect each other’s
performance. Friction in the managed lanes occurs at higher general
purpose lane densities, when no physical separation is provided
between the general purpose and managed lanes. Cross weave occurs
when traffic from a general purpose on-ramp segment must cross
multiple general purpose lanes to access the managed lane at a nearby
ramp or access segment, thereby affecting general purpose segment
capacity.

Access to and from a managed lane can occur in one of three ways,
depicted in Exhibit 10-3 and described below:

   - _At-grade lane-change access_ occurs when managed lane traffic
enters the general purpose lanes through a conventional on-ramp
roadway (from the right), cross-weaves across multiple general
purpose lanes, and enters the managed lane facility. Managed lane
traffic exits in the same segment, so this configuration also results in a
form of weaving movement. This access strategy is common for
concurrent managed lane facilities. Access between managed and


general purpose lanes is sometimes constrained to specific locations
or openings, which affects the weaving intensity at these access
points. This access configuration requires a _cross-weaving_ movement
across general purpose lanes for drivers to position themselves in
advance of the access point and a _lane-change or weaving movement_
to get from the general purpose lanes into the managed lanes.

- _At-grade ramp access_ occurs where managed lane traffic enters the
general purpose lanes through a conventional on-ramp roadway (from
the right). Entering and exiting traffic may cross-weave across
multiple general purpose lanes, similar to the first case, but the
entrance to (or exit from) the managed lane facility is confined to an
_at-grade_ on-ramp or off-ramp. Operationally, the general purpose
lanes may be affected by the cross-weaving flow. The managed lane
operations, in turn, are only affected by merging and diverging
maneuvers at the access points at the ramps. This access
configuration requires a _cross-weaving_ movement across the general
purpose lanes for drivers to position themselves in advance of the
access point and a _ramp merge movement_ to get from the general
purpose lanes into the managed lanes.

- _Grade-separated ramp access_ occurs where the managed lanes are
accessed on a grade-separated structure (i.e., bridge or underpass).
The operational impact on the general purpose lanes is minimal in
this case, because the cross-weaving movement is eliminated. The
managed lanes are affected by friction from the entering or exiting
ramp flows in the same fashion as the general purpose lanes. This
access configuration does not require any cross-weaving across the
general purpose lanes because of the grade-separated ramp, and the
access to the managed lanes is handled by a _ramp merge movement_ .
If all managed lane access is grade-separated, the result effectively is
an entirely separate facility. However, a mix of grade-separated and
at-grade access points is common.


**Exhibit 10-3: Typology of Managed Lane Access Point Designs**


(a) At-Grade Lane-Change Access


(b) At-Grade Ramp Access


(c) Grade-Separated Ramp Access


Notes: GP = general purpose, ML = managed lane.


The spatial extent of the _access point influence area_ (APIA) for gradeseparated ramp access is defined in the HCM’s ramp merge and diverge
methodology. The ramp influence area for general purpose facilities is
defined to be 1,500 ft from the ramp gore for both on-ramps (measured
downstream) and off-ramps (measured upstream). The APIA for gradeseparated managed lane access points follows the same convention.

The intensity and impact of the _cross-weaving flows_ between a general
purpose ramp and the access region between the general purpose and
managed lanes need to be analyzed for both at-grade access types. The
minimum cross-weave length is defined as the distance between the closest


upstream general purpose on-ramp gore and the start of the managed lane
ramp or access opening [see for example, Exhibit 10-3(b)]. The maximum
cross-weave length is defined as the distance from the ramp gore to the end
of the access opening; this maximum does not apply to at-grade on-ramp
access.


**FREE-FLOW SPEED**

_Free-flow speed (FFS)_ is the average speed of vehicles on a given
segment or facility, measured under low-volume conditions, when drivers
are free to drive at their desired speed and are not constrained by the
presence of other vehicles. FFS is considered the theoretical speed when
both density and flow rate are zero. FFS is an important characteristic, since
the capacity _c_, service flow rates _SF_, service volumes _SV_, and daily service
volumes _DSV_ depend on it.

Chapter 12, Basic Freeway Segments, presents speed–flow curves that
indicate that, under base conditions, the FFS on freeways is expected to
prevail at flow rates below 1,000 passenger cars per hour per lane (pc/h/ln).
In this range, speed is insensitive to flow rate. This characteristic simplifies
and allows measurement of free-flow speeds directly in the field and from
sensor data.

However, there are exceptions where speeds are affected even at low
flow rates. For example, speeds may be reduced if there is significant truck
presence or if truck speeds are posted lower than passenger car speeds.
Similarly, speeds on single-lane managed lane facilities have been shown to
decline immediately even at low flow rates, rather than being stable until a
breakpoint is reached. Under these conditions, the FFS becomes more of a
theoretical concept that can be difficult to measure directly in the field.

Chapter 12 presents a methodology for estimating the FFS of a basic
freeway segment if it cannot be measured directly. The FFS of a basic
freeway segment is sensitive to three variables:

   - Lane widths,

   - Lateral clearances, and

   - Total ramp density.


The most critical of these variables is total ramp density. _Total ramp_
_density_ is defined as the average number of on-ramp, off-ramp, major merge,
and major diverge junctions per mile in the analysis direction (one side of
the freeway only). Freeway interchanges can have two (standard diamond),
three (partial cloverleaf), or four (full cloverleaf) ramps in the analysis
direction. For segment analyses, ramp density is computed for a 6-mi section
centered on the segment’s midpoint; however, for facility analyses, ramp
density is calculated across the entire facility (i.e., total number of ramps
divided by total facility length).

While the methodology for determining FFS is provided in Chapter 12,
Basic Freeway and Multilane Highway Segments, it is also applied in
Chapter 13, Freeway Weaving Segments, and Chapter 14, Freeway Merge
and Diverge Segments. Thus, FFS affects the operation of all basic, weaving,
merge, and diverge segments on a freeway facility.

Exhibit 10-4 illustrates the determination of total ramp density on a 9-mi
length of a directional freeway facility. As shown, four ramp terminals are
located along the 9-mi facility; therefore, the total ramp density is 4/9 = 0.44
ramps/mi.


**Exhibit 10-4: Sample Facility with Five Sections**


**FREEWAY FACILITY CAPACITY**

Capacity traditionally has been defined for uniform segments of roadway,
traffic, and control conditions. When facilities consisting of a series of
connected segments are considered, the concept of capacity is more nuanced.

The methodologies of Chapters 12, 13, and 14 allow the capacity of each
basic, weaving, merge, and diverge segment to be estimated. Since all
segments of a facility are highly unlikely to have the same roadway, traffic,


and control conditions and even less likely to have the same capacity, the
freeway facility capacity hinges on the identification of the critical
segment(s) where the breakdown starts. A definition based on this concept is
provided below.


**Conceptual Approach to Understanding Capacity and Flow**
**Regimes on a Freeway Facility**

Consider the sample facility shown in Exhibit 10-4 and the associated
data in Exhibit 10-5 below. This example illustrates five consecutive
sections that are to be analyzed as one “freeway facility.” Note that while
this conceptual example is illustrated by using sections, the methodology in
fact performs computations at the segment level.


**Exhibit 10-5: Example of the Effect of Capacity on Demand and Actual Flow Rates on a**
**Freeway Facility**










|Scenario|Performance<br>Measures|Freeway Section<br>1 2 3 4 5|
|---|---|---|
|Scenario 1<br>(stable flow)|Demand_ vd_<br>(veh/h)<br>Capacity_ c_ (veh/h)<br>Volume_ va_ (veh/h)<br>_vd/c_ ratio<br>_va/c_ ratio<br>Demand_ vd_<br>(veh/h)<br>Capacity_ c_ (veh/h)<br>Volume_ va_ (veh/h)<br>_vd/c_ ratio<br>_va/c_ ratio<br>Demand_ vd_<br>(veh/h)<br>Capacity_ c_ (veh/h)<br>Volume_ va_ (veh/h)<br>_vd/c_ ratio<br>_va/c_ ratio|3,400 3,500 3,400 4,200 4,400<br>4,000 4,000 4,500 4,500 4,500<br>3,400 3,500 3,400 4,200 4,400<br>0.850 0.875 0.756 0.933 0.978<br>0.850 0.875 0.756 0.933 0.978|
|Scenario 2<br>(add 200 veh/h to each section)|Scenario 2<br>(add 200 veh/h to each section)|3,600 3,700 3,600 4,400 4,600<br>4,000 4,000 4,500 4,500 4,500<br>3,600 3,700 3,600 4,400 4,500<br>0.900 0.925 0.800 0.978 1.022<br>0.900 0.925 0.800 0.978 1.000|
|Scenario 3<br>(increase Scenario 1 demand by<br>10% in all sections)|Scenario 3<br>(increase Scenario 1 demand by<br>10% in all sections)|3,740 3,850 3,740 4,620 4,840<br>4,000 4,000 4,500 4,500 4,500<br>3,740 3,850 3,740 4,500 4,500<br>0.935 0.963 0.831 1.027 1.076<br>0.935 0.963 0.831 1.000 1.000|



Note: Shaded cells indicate segments where demand exceeds capacity.


Demand flow rates _vd_, capacities _c_, and actual served (or observed) flow
rates _va_ are given, as are the resulting _vd/c_ and _va/c_ ratios. An increase in
capacity is observed in Sections 3 to 5. The example covers three conceptual
scenarios that illustrate the difference between _demand flow rate_ and _actual_
_served flow rate,_ as a result of section capacity constraints that meter the full
demand.

In Scenario 1, none of the demand flow rates exceeds the section
capacities on the facility. Thus, no breakdowns occur, and the actual volume
served is the same as the demand flow rates (i.e., _vd_ = _va_ for this scenario).
None of the _vd/c_ ratios exceeds 1.00, although relatively high ratios (0.978)
occur in Section 5. Operating conditions in scenarios such as Scenario 1
imply a stable flow regime.

Scenario 2 adds 200 veh/h of through demand to each section. In this
case, Section 5 experiences a breakdown since its demand flow rate exceeds
its capacity. In this section, the actual flow rate _va_ differs from the demand
flow rate _vd_, since the actual flow rate can never exceed the section capacity
_c_ .

In Scenario 3, all demand flow rates in Scenario 1 are increased by 10%.
In this case, the demand flow rate exceeds capacity in both Sections 4 and 5.
Again, the demand and actual flow rates will differ in these sections.
Operating conditions under Scenario 2 or 3 are considered to be unstable,
with breakdown and propagation of queuing conditions upstream likely to
occur.

This example highlights a number of important points concerning the
analysis of freeway facilities:

1. In applications of the methodology, it is critical that the difference
between _demand_ flow rate _vd_ and _actual_ flow rate _va_ be highlighted
and that both values be clearly and appropriately labeled. The actual
flow rate can never exceed the section or segment capacity.

2. It might be argued that the analysis of Scenario 1 above is sufficient
to understand the facility’s operation as long as all its segments are
undersaturated (i.e., all sections’ _vd/c_ ratios are less than or equal to


1.00). However, when _any section’s vd/c_ ratio exceeds 1.00, such a
simple analysis ignores the propagation of queues in space and time.

3. The analysis shown in Exhibit 10-5 for Scenarios 2 and 3 is
incomplete. In Scenario 2, when Section 5 breaks down, queues will
begin to form and propagate upstream. Thus, even though the demands
in Sections 1 through 4 are less than those segments’ capacities, the
queues generated by Section 5 will propagate over time, possibly all
the way to Segment 1, and thus could significantly affect upstream
operations. In Scenario 3, Sections 4 and 5 fail. Their queues will
also propagate upstream over time and alter the actual flow rates _va_
on the affected segments.

4. In Scenarios 2 and 3, sections downstream of Section 5 are also
affected, since the demand flow rate is prevented from reaching those
sections by the capacity constraint on Section 5 (and Section 4 in
Scenario 3).

5. In this example, the sections that break down first do not necessarily
have the lowest capacities. Breakdown occurs first in Section 5,
which has one of the higher capacities on the facility but also the
highest flow rate and, therefore, the highest demand-to-capacity ratio.


**Capacity in the Context of Freeway Facilities**

In view of all these issues, the notion of capacity on a freeway facility
can be described as follows:

_Freeway facility capacity is governed by the position and severity of_
_active bottlenecks (i.e., segments with vd/c > 1.0) along its length. Both_
_characteristics vary over time and space, depending on the time-_
_varying demand flow rates on each facility segment. A bottleneck that_
_is active at one time may hide another (less severe) bottleneck further_
_downstream by suppressing demand flows to that downstream_
_bottleneck. In short, there is no simple definition for freeway facility_
_capacity, other than it is variable over time and influenced by the_
_timing and location of active bottlenecks._

The _critical segment_ is generally defined as the bottleneck segment that
will break down the earliest, given that all traffic, roadway, and control


conditions do not change, including the spatial distribution of demands on
each component segment. This definition is not a simple one. It depends on
the relative demand characteristics and, as stated earlier, _can change over_
_time_ as the demand pattern changes. Facility capacity may be different from
the capacity of the component segment with the lowest capacity. Therefore,
the evaluation of individual segment demands and capacities is important. In
fact, the methodology explained later in this chapter specifies that the facility
be assigned LOS F in any time interval in which any segment demand-tocapacity ratio exceeds 1.00.


**Active and Hidden Bottlenecks**

The freeway facilities methodology is able to identify both _active_ and
_hidden_ bottlenecks. An a _ctive bottleneck_ is defined as a segment with a
demand-to-capacity ratio ( _vd/c_ ) greater than 1.0, an actual flow-to-capacity
ratio ( _va/c_ ) equal to 1.0, and queuing upstream of the bottleneck segment.
Active bottlenecks are the congestion points best known to operating
agencies and are of critical importance in validating the procedure to match
field-observed conditions. The actual flow at an active bottleneck is metered
by its capacity, resulting in downstream segments likely having _va/c_ ratios
that are less than their _vd/c_ ratios.

A _hidden bottleneck_ is defined as a segment with a demand-to-capacity
ratio ( _vd/c_ ) greater than 1.0 but an actual flow-to-capacity ratio ( _va/c_ )
typically less than 1.0 (or equal to 1.0 in some cases), with no queues
forming upstream of the segment. In other words, hidden bottlenecks are
segments with _vd/c_ greater than 1.0 where the demand is metered by a more
severe active bottleneck upstream. Since a portion of the true segment
demand is stored in the upstream queue, the actual flow arrivals at the hidden
bottleneck may be less than 1.0 and no queues are formed.

Knowledge of hidden bottlenecks is of primary importance when
improvement strategies for a congested facility are evaluated. For example,
if an analysis points to an active bottleneck, the operating agency may decide
to improve operations by widening the bottleneck segment. However, if one
or more hidden bottlenecks are located downstream of that segment, the
improvements may simply result in congestion migrating downstream. For


true removal of the congestion, an agency may need to improve all active and
hidden bottlenecks.


**Prebreakdown and Queue Discharge Capacity**

The term _capacity,_ as used thus far, refers to the critical segment capacity
—the flow rate that immediately precedes the onset of the breakdown.
Chapter 12 defines breakdown as a sudden drop in speed of at least 25%
below the free-flow speed for a sustained period of at least 15 min that
results in queuing upstream of the bottleneck. Thus, the segment capacities
shown in Exhibit 10-5 are also called _prebreakdown_ capacities or flow
rates, defined as the 15-min average flow rate immediately preceding the
breakdown event.

Once the breakdown takes place and queues begin to form, the flow rates
discharging from the queue at the bottleneck are generally lower than the
prebreakdown capacity. This postbreakdown flow rate or _queue discharge_
_flow rate_ is defined as the 15-min flow rate during oversaturated conditions
(i.e., during the time interval after breakdown and before recovery). The
difference in flow rate varies considerably in the research literature, from a
value of zero (and sometimes negative values) up to 15% to 20% ( _7_ ). The
amount of the drop was also found to depend on the magnitude of the
prebreakdown flow rate.

A synthesis of the literature indicates that an average value for the
capacity drop is about 7% ( _7_ ). This reduced capacity is used in the
oversaturated analysis procedure to estimate the rate at which queues will
form and dissipate once demand exceeds capacity. When the queue is
cleared, the segment’s original (prebreakdown) capacity is restored. Details
of the two-capacity phenomenon, and its application in the core
computational methodology, are explained in the next section. A formal
definition of freeway segment capacity is provided in Chapters 12, and a
measurement method is provided in Chapter 26.


**LOS: COMPONENT SEGMENTS AND THE FREEWAY FACILITY**


**LOS of Component Segments**


Chapters 12, 13, and 14 provide methodologies for determining the LOS
in basic, weaving, merge, and diverge segments on the basis of the segment’s
average density. In all cases, LOS F is also defined when _vd/c_ is greater than
1.00.

This chapter’s methodology provides an analysis of breakdown
conditions, including the spatial and time impacts of a breakdown. Thus, in
the performance of a facility-level analysis, LOS F in a component segment
can be identified ( _a_ ) when the segment _vd/c_ is greater than 1.00 and ( _b_ ) when
a queue resulting from a downstream breakdown extends into an upstream
segment. The latter cannot be estimated by using the individual segment
analysis procedures of Chapters 12–14.

Thus, when a facility-level analysis is performed, LOS F for a
component segment will be identified in two complementary ways:

   - When _vd/c_ is greater than 1.00 for one or more critical segments, or

   - When the segment density is greater than 45 pc/mi/ln for basic
freeway segments or 43 pc/mi/ln for weaving segments.

The latter condition identifies segments in which queues have formed as
a result of downstream breakdowns.


**LOS for a Freeway Facility**

Because LOS for basic, weaving, merge, and diverge segments on a
freeway is defined in terms of density, LOS for a freeway facility is also
defined on the basis of density. The method distinguishes between density
thresholds used to designate LOS on urban and rural freeway facilities on the
basis of research ( _8_ ). Such a distinction in LOS is not made at the segment
level.

The classification of a facility as urban or rural is made on the basis of
the Federal Highway Administration’s smoothed or adjusted urbanized
boundary definition ( _9_ ), which in turn is derived from Census data. Facilities
that fall fully within an urban area or fully outside of it are classified as
urban or rural, respectively. If a freeway facility crosses an urbanized area
boundary, analyst judgment is needed in classifying it as urban or rural.


Generally, the entire length of the facility needs to be assigned the same area
type.

A facility analysis will result in a density determination and LOS for
each component segment. The facility LOS will be based on the weighted
average density for all segments within the defined facility. Weighting is done
on the basis of segment length and number of lanes in each segment, in
accordance with Equation 10-1:


**Equation 10-1:**


where

_DF_ = average density for the facility in a given 15-min analysis period
(pc/mi/ln),

_Di_ = density for segment _i_ (pc/mi/ln),

_Li_ = length of segment _i_ (mi),

_Ni_ = number of lanes in segment _i_, and

_n_ = number of segments in the defined facility.


LOS criteria for urban and rural freeway facilities are shown in Exhibit
10-6. Urban LOS thresholds are the same density-based criteria used for
basic freeway segments. Studies on LOS perception by rural travelers
indicate the presence of lower-density thresholds in comparison with their
urban freeway counterparts. The average LOS applies to a specific analysis
period, usually 15 min.


**Exhibit 10-6: LOS Criteria for Urban and Rural Freeway Facilities**


|LOS|Freeway Facility Density (pc/mi/ln)<br>Urban Rural|
|---|---|
|||


A LOS descriptor for the overall freeway facility must be used with care.
The LOS for individual segments composing the facility should also be
reported. The overall LOS, being an average, may mask serious problems in
individual segments of the facility.

This is particularly important if one or more of the component segments
are operating at LOS F. As described in this chapter’s methodology section,
the freeway facility methodology applies models to estimate the propagation
of the effects of a breakdown in time and space. Where breakdowns occur in
one or more segments of a facility, the average LOS is of limited use.

For urban freeway facilities, LOS A through E are defined on the basis of
the same densities that apply to basic freeway segments. Rural freeway
facilities have lower density thresholds, as indicated in Exhibit 10-6. This
difference is a result of rural motorists’ higher LOS expectations. The analyst
is cautioned that a rural facility analysis may produce LOS F without any of
its segments experiencing breakdown ( _vd/c_ - 1). As a result, LOS F for a
facility represents a case in which any component segment of the freeway has
a _vd/c_ ratio that exceeds 1.00 or in which the average density of the study
facility exceeds 45 pc/mi/ln (for urban freeways) or 39 pc/mi/ln (for rural
freeways). This chapter’s methodology allows the analyst to map the impacts
of breakdowns in time and space, and close attention to the LOS of
component segments is necessary.


# **3. METHODOLOGY**

This chapter’s methodology provides for the integrated analysis of a
freeway facility composed of connected segments. The methodology builds
on the models and procedures for individual segments described in Chapter
12, Basic Freeway and Multilane Highway Segments; Chapter 13, Freeway
Weaving Segments; and Chapter 14, Freeway Merge and Diverge Segments.


**SCOPE OF THE METHODOLOGY**

Because the freeway facility methodology builds on the segment
methodologies of Chapters 12, 13, and 14, it incorporates all aspects of those
chapters’ methodologies. This chapter’s method adds the ability to analyze
operations when LOS F exists on one or more segments of the study facility.
It draws from research sponsored by the Federal Highway Administration
( _10_ ).

In Chapters 12–14, the existence of a breakdown (LOS F) is identified
for a given segment, as appropriate. However, the segment methodologies do
not provide tools for analyzing the impacts of such breakdowns over time
and space.

The methodology analyzes a set of connected segments over a set of
sequential 15-min periods. In deciding which segments and analysis periods
to analyze, two principles should be observed:

1. The first and last segments of the defined facility should _not_ operate
at LOS F.

2. The first and last analysis periods of the analysis should _not_ include
any segments that operate at LOS F.

When the first segment operates at LOS F, a queue extends upstream that
is not included in the facility definition and that therefore cannot be analyzed.
The first segment should thus be long enough to contain the queue, although
this may not always be practical or possible. When a queue does extend


beyond the first segment, the methodology reports the number of unserved
vehicles that should be considered by the analyst.

When the last segment operates at LOS F, there may be a downstream
bottleneck outside the facility definition. Again, the impact of this congestion
cannot be evaluated because it is not contained within the defined facility.
LOS F during either the first or the last analysis period creates similar
problems with regard to time. If the first analysis period operates at LOS F,
LOS F may exist in previous analysis periods as well. If the last analysis
period is at LOS F, subsequent periods may also operate at LOS F. The
impact of a breakdown cannot be fully analyzed unless the queuing is
contained within the defined facility and defined analysis period. The same
problems would exist if the analysis were performed by using simulation.


**Spatial and Temporal Limits**

There is no limit to the number of analysis periods that can be analyzed.
The temporal extent should be sufficiently long to contain the formation and
dissipation of all queues as discussed above. Ideally, 30 min of analysis time
should be added before and after the known peak period for a clear picture
of the onset and dissipation of congestion.

The length of the freeway should be less than the distance a vehicle
traveling at the average speed can achieve in 15 min. This specification
generally results in a maximum facility length between 9 and 12 mi.
Facilities longer than these limits should be divided into subfacilities at
appropriate breakpoints. Each subfacility can then be analyzed separately
with this chapter’s procedure.


**Performance Measures**

The core freeway facility methodology generates the following
performance measures for each segment and analysis period being evaluated:

   - Capacity,

   - FFS,

   - Demand-to-capacity and volume-to-capacity ratios,

   - Space mean speed,


   - Average density,

   - Travel time (min/veh),

   - Vehicle miles traveled (VMT, demand and volume served),

   - Vehicle hours of travel (VHT),

   - Vehicle hours of delay (VHD), and

   - Motorized vehicle LOS for each component segment and for the
facility.

In addition, space mean speed, average density, travel time, VMT, VHT,
VHD, and LOS are aggregated in each time interval across all segments in
the facility. Performance measures are not aggregated across analysis
periods. Details on how this aggregation is performed are given in Chapter
25.


**Strengths of the Methodology**

The following are strengths of the freeway facilities methodology:

1. The methodology captures oversaturated and undersaturated
conditions in an extended time–space domain.

2. The methodology accounts for all active and hidden mainline
bottlenecks and can be used to evaluate the operational effects of
control strategies and capacity improvements along the facility.

3. The methodology explicitly tracks queues as they form and dissipate
across segments and time intervals.

4. The methodology allows for time-varying demands and capacities,
thereby permitting the evaluation of control strategies that affect
demand (e.g., traffic diversion or peak spreading) or capacity (e.g.,
hard running shoulders, lane additions, ramp metering).

5. The methodology can account for the effects of short-term incidents,
weather events, and work zones.

6. The methodology is consistent with the segment methodologies in

Chapters 12, 13, and 14 if all _vd/c_ ratios are less than or equal to 1.0,


and it properly accounts for the interaction of segments when any _vd/c_
ratio is greater than 1.0.

Given enough time, a completely undersaturated time–space domain can
be analyzed manually, although the process can be difficult and timeconsuming. It is not expected that manual analysis of a time–space domain
that includes oversaturation will ever be carried out. A computational engine
is needed to implement the methodology, regardless of whether the time–
space domain contains oversaturated segments and analysis periods. The
engine is available in the online HCM Volume 4 for research purposes but
should not be used for commercial applications.


**Limitations of the Methodology**

The methodology has the following limitations:

1. The methodology does not account for delays caused by vehicles
using alternative routes or vehicles leaving before or after the
analysis period.

2. Multiple overlapping breakdowns or bottlenecks are difficult to
analyze and cannot be fully evaluated by this methodology. Other
tools may be more appropriate for specific applications beyond the
capabilities of the methodology. Consult Chapter 6, HCM and
Alternative Analysis Tools, for a discussion of simulation and other
models.

3. Spatial, temporal, modal, and total demand responses to traffic
management strategies are not automatically incorporated into the
methodology. On viewing the facility traffic performance results, the
analyst can modify the demand input manually to analyze the effect of
user-demand responses and traffic growth. The accuracy of the
results depends on the accuracy of the estimation of user-demand
responses.

4. The completeness of the analysis will be limited if any freeway
segment in the first or last time interval, or the first or last freeway
segment in any analysis period, has a demand-to-capacity ratio
greater than 1.00.


5. The method does not address conditions in which off-ramp capacity
limitations result in queues that extend onto the freeway or affect the
behavior of off-ramp vehicles.

6. Because this chapter’s methodology incorporates the methodologies
for basic, weaving, merging, and diverging freeway segments, the
limitations of those procedures also apply here.

7. The method does not include analysis of the streetside terminals of
freeway on- and off-ramps. The methodologies of Chapters 19, 20,
21, and 22 should be used for intersections that are signalized, twoway STOP-controlled, all-way STOP-controlled, and roundabouts,
respectively. Chapter 23, Ramp Terminals and Alternative
Intersections, provides a more comprehensive analysis of freeway
interchanges where the streetside ramp terminals are signalized
intersections or roundabouts.


**REQUIRED DATA AND SOURCES**

The analysis of a freeway facility requires details concerning each
segment’s geometric characteristics, as well as each segment’s demand
characteristics during each analysis period. Exhibit 10-7 shows the data
inputs that are required for an operational analysis of a freeway facility,
potential sources of these data, and suggested default values.


**Exhibit 10-7: Required Input Data, Potential Data Sources, and Default Values for**
**Freeway Facility Analysis**


**Required Data and Units** **Potential Data Source(s)** **Suggested Default Value**

_Geometric Data_

Direct speed measurements, Base free-flow speed:
**Free-flow speed** (mi/h) estimate from FFS prediction speed limit + 5 mi/h (range
algorithm 55–75 mi/h)

Segment and section length
Road inventory, aerial photo Must be provided
(ft)

_**Number of mainline freeway**_
Road inventory, aerial photo At least 2
_**lanes**_ (one direction)

Lane width (ft) Road inventory, aerial photo 12 ft (range 10–12 ft)
Right-side lateral clearance (ft) Road inventory, aerial photo 6 ft (range 0–6 ft)

Total ramp density in analysis Must be provided
Road inventory, aerial photo
direction (range 0–6 ramps/mi)


**Area type** (urban, rural) Road inventory, aerial photo Must be provided

Terrain type Design plans, analyst
Must be provided
(level, rolling, specific grade) judgment

Ramp number of lanes Road inventory, aerial photo 1 lane
Ramp acceleration or
Road inventory, aerial photo 500 ft
deceleration lane length (ft)

Ramp free-flow speed (mi/h) Road inventory, aerial photo 35–45 mi/h
_**Geometry of managed lanes**_ Road inventory, aerial photo Must be provided

_Demand Data_
_**Mainline entry demand**_
_**volume by time interval**_ Field data, modeling Must be provided
(veh/h)

_**On-ramp and off-ramp**_
_**demands by time interval**_ Field data, modeling Must be provided
(veh/h)

_**Weaving demands on**_
_**weaving segments by time**_ Field data, modeling Must be provided
_**interval**_ (veh/h)

Heavy vehicle percentage (%) Field data 5% (urban), 12% (rural) _[a]_

Driver population speed and
1.00 (see Chapter 26 for
capacity adjustment factors Field data
details)
(decimal)

_**Jam density**_ (pc/mi/ln) Field data 190 (range 150–270)

_**Queue discharge capacity**_
Field data 7% (range 0%–20%)
_**drop**_ (%)

_**Managed lane demand**_
Field data, modeling Must be provided
_**volume**_ (veh/h)


Notes: _**Bold italic**_ indicates high sensitivity (>20% change) of service measure to the choice
of default value.
**Bold** indicates moderate sensitivity (10%–20% change) of service measure to the
choice of default value.
_a_ See Chapter 26 in Volume 4 for state-specific default heavy vehicle percentages.


Where any data item is not readily available or collectible, the analysis
may be supplemented by using consistent default values for each segment.
Lists and discussions of default values are found in Chapters 12, 13, and 14
for basic freeway, weaving, and merge and diverge segments, respectively.


**OVERVIEW**


The freeway facility methodology represents one of three parts in an
evaluation sequence that can also include a freeway reliability analysis and
an evaluation of ATDM strategies. Part A: Core Freeway Facility Analysis
(single study period) is presented in this chapter, while Parts B and C are
presented in Chapter 11, Freeway Reliability Analysis _._ Part A constitutes the
core methodology; Parts B and C represent adaptations and extensions of the
methodology for reliability and ATDM assessment, respectively. On
completion of the 17 computational steps in the core methodology, the analyst
may decide to continue to perform reliability and ATDM analyses by using
the procedures described in Chapter 11.

Exhibit 10-8 summarizes the process of implementing the core
methodology for analyzing freeway facilities. The methodology adjusts
vehicle speeds appropriately to account for the impacts of adjacent upstream
or downstream segments. The methodology can analyze freeway traffic
management strategies only in cases where 15-min intervals (or their
multiples) are appropriate and when reliable data for estimated capacity and
demand exist.


**Exhibit 10-8: Freeway Facility Methodology**


Note: ML = managed lane, LQ = length of queue.


**COMPUTATIONAL STEPS**

This section describes the methodology’s computational modules. To
simplify the presentation, the focus is on the function of and rationale for
each module. Chapter 25 presents an expanded version of this section,
including all the supporting analytical models and equations.


**Step A-1: Define Study Scope**

In this initial step, the analyst defines the spatial extent of the facility
(start and end points, total length) and the temporal extent of the analysis
(number of 15-min analysis periods). The analyst should further decide
which study extensions (if any) apply to the analysis (i.e., managed lanes,
reliability, ATDM).

A time–space domain for the analysis must be established. The domain
consists of a specification of the freeway _sections_ and _segments_ included in
the defined facility and an identification of the time intervals for which the
analysis is to be conducted. For the freeway facility shown in Exhibit 10-9, a
typical time–space domain is shown in Exhibit 10-10.


**Exhibit 10-9: Example Freeway Facility for Time–Space Domain Illustration**


**Exhibit 10-10: Example Time–Space Domain for Freeway Facility Analysis**

|Analysis<br>Period|Seg<br>1|Seg<br>2|Seg<br>3|Seg<br>4|Seg<br>5|Seg<br>6|Seg<br>7|Seg<br>8|Seg<br>9|Seg<br>10|Seg<br>11|Seg<br>12|
|---|---|---|---|---|---|---|---|---|---|---|---|---|
|1|||||||||||||
|2|||||||||||||
|3|||||||||||||
|4|||||||||||||
|5|||||||||||||
|6|||||||||||||
|7|||||||||||||
|8|||||||||||||



Note: Seg = segment.


The horizontal scale indicates the distance along the freeway facility. A
freeway _section_ boundary occurs where there is a change in demand—that is,
at each on-ramp or off-ramp. These areas are referred to as _sections_ because
adjustments will be made within the procedure to determine where _segment_
boundaries should be for analysis. This process relies on the influence areas
of merge, diverge, and weaving segments, discussed earlier in this chapter,
and on variable length limitations specified in Chapter 13 for weaving
segments and in Chapter 14 for merge and diverge segments. The facility in
Exhibit 10-9 corresponds to seven sections that are then divided into 12
segments. The time–space domain is illustrated at the segment level, which is
the basis of the HCM methodology. However, aggregation to the section level
is possible and may be needed to compare the results with field data
available only at the section level.

The longer the facility length without congestion on the horizontal scale,
the more the congested travel times are diluted (see Equation 10-1). The
analyst should avoid overly long segments at the edges of the space domain


when possible, to avoid diluting the overall results. However, the first
segment should be long enough to contain all queues, if practical.

The vertical scale indicates the study duration. Time extends down the
time–space domain, and the scale is divided into 15-min intervals. In the
example shown, there are 12 segments and 8 analysis periods, yielding 12 ×
8 = 96 time–space _cells_, each of which will be analyzed within the
methodology. The analysis could be extended to up to a 24-h analysis,
corresponding to ninety-six 15-min analysis periods.

The boundary conditions of the time–space domain are extremely
important. The time–space domain will be analyzed as an independent
freeway facility having no interactions with upstream or downstream
portions of the freeway or with any connecting facilities, including other
freeways and surface facilities. Therefore, no congestion should occur along
the four boundaries of the time–space domain. The cells located along the
four boundaries should all have demands less than capacity and should
contain undersaturated flow conditions. A proper analysis of congestion
within the time–space domain can occur only if the congestion is limited to
internal cells not along the time–space boundaries. If necessary, the analysis
domain should be extended in time, space, or both to contain all congestion
effects.


**Step A-2: Divide Facility into Sections and Segments**

In this step, the analyst first defines the number of sections from gore
point to gore point along the selected facility. These gore-to-gore sections
are more consistent with modern freeway performance databases than are
HCM segments, and this consistency is critical for calibrating and validating
the freeway facility.

The analyst later divides sections into HCM segments (basic, merge,
diverge, weave, overlapping ramp, or managed lane segment) as described
below. Judgment may be needed for segments that do not cleanly fit the HCM
definitions. The first and last segment must always be a basic segment, and
these may be considered as “half sections,” since only one gore point is
included in each. This point was illustrated previously in Exhibit 10-2 and
Exhibit 10-9.


When a facility includes managed lanes, this step also includes defining
parallel lane groups for managed lanes and general purpose lanes, as will be
described in Section 4.

The sections of the defined freeway facility are bounded by points where
demand changes. However, this approach does _not_ fully describe individual
_segments_ for analysis within the methodology. The conversion from sections
to analysis segments can be performed manually by applying the principles
discussed here, along with those given previously in the Facility
Segmentation Guidance subsection of Section 2.

Chapter 14, Freeway Merge and Diverge Segments, indicates that each
merge segment extends from the merge point to a point 1,500 ft downstream
of it. Each diverge segment extends from the diverge point to a point 1,500 ft
upstream of it. This allows for a number of scenarios affecting the definition
of analysis segments within the defined freeway.

Consider the illustration of Exhibit 10-11. It shows a one-lane on-ramp
followed by a one-lane off-ramp with no auxiliary lane between them. The
illustration assumes that there are no upstream or downstream ramps or
weaving segments that impinge on this section.

In Exhibit 10-11(a), the two ramps are 4,000 ft apart. The merge segment
extends 1,500 ft downstream from the on-ramp while the diverge segment
extends 1,500 ft upstream from the off-ramp, which leaves a 1,000-ft basic
freeway segment between them.

In Exhibit 10-11(b), the two ramps are 3,000 ft apart. The two 1,500-ft
ramp influence areas define the entire length. Therefore, there is no basic
freeway segment between the merge and diverge segments.

In Exhibit 10-11(c), the situation is more complicated. With only 2,000 ft
between the ramps, the merge and diverge influence areas overlap for a
distance of 1,000 ft.


**Exhibit 10-11: Defining Analysis Segments for a Ramp Configuration**


(a) Section Length Between Ramps = 4,000 ft


(b) Section Length Between Ramps = 3,000 ft


(c) Section Length Between Ramps = 2,000 ft


Chapter 14 covers this situation. Where ramp influence areas overlap, the
analysis is conducted for each ramp separately. The analysis producing the


worse LOS (or service measure value if the LOS is equivalent) is used to
define operations in the overlap area.

The facility methodology goes through the logic of distances and segment
definitions to convert section boundaries to segment boundaries for analysis.
If the distance between an on-ramp and an off-ramp is less than the full
influence area of 1,500 ft, the worst case is applied to the distance between
the ramps, while basic segment criteria are applied to segments upstream of
the on-ramp and downstream of the off-ramp.

A similar situation can arise where weaving configurations exist. Exhibit
10-12 illustrates a weaving configuration within a defined freeway facility.
In this case, the distance between the merge and diverge ends of the
configuration must be compared with the maximum length of a weaving
segment _LwMAX_ . If the distance between the merge and diverge points is less
than or equal to _LwMAX_, the entire segment is analyzed as a weaving segment,
as shown in Exhibit 10-12(a).


**Exhibit 10-12: Defining Analysis Segments for a Weaving Configuration**


(a) Case I: _LB_ = _LwMAX_ (weaving segment exists)


(b) Case II: _LB_            - _LwMAX_ (analyze as basic segment)


Three lengths are involved in analyzing a weaving segment:

   - The _short length_ of the segment, defined as the distance over which
lane changing is not prohibited or dissuaded by markings ( _LS_ );

   - The _base length_ of the segment, measured from the points where the
edges of the travel lanes of the merging and diverging roadways
converge ( _LB_ ); and

   - The _influence area_ of the weaving segment ( _LWI_ ), which includes
500 ft upstream and downstream of _LB_ .

The influence area is the length that is used in all the predictive models
for weaving segment analysis. However, the results of these models apply to
a distance of _LB_ + 500 ft upstream to _LB_ + 500 ft downstream. For further
discussion of the various lengths applied to weaving segments, consult
Chapter 13.

If _LS_ is greater than _LwMAX_, the merge and diverge segments are too far
apart to form a weaving segment. As shown in Exhibit 10-12(b), the segment
is treated as a basic freeway segment.

In the Chapter 13 weaving methodology, the value of _LwMAX_ depends on a
number of considerations, including the split of component flows, demand
flows, and other traffic factors. A weaving configuration could therefore
qualify as a weaving segment in some analysis periods and as a separate
merge, diverge, or basic segment in others.

In segmenting the freeway facility for analysis, merge, diverge, and
weaving segments are identified as illustrated in Exhibit 10-11 and Exhibit
10-12. All segments not qualifying as merge, diverge, or weaving segments
are basic freeway segments.

However, a long basic freeway section may have to be divided into
multiple segments. This situation occurs when there is a sharp break in
terrain within the section. For example, a 5-mi section may have a constant
demand and a constant number of lanes. If there is a 2-mi level terrain
portion followed by a 4% grade that is 3 mi long, the level terrain portion
and the specific grade portion would be established as two separate
consecutive basic freeway segments.


**Step A-3: Input Data**

Demand, geometry, and other data must be specified. Since the
methodology builds on segment analysis, all data for each segment and each
analysis period must be provided, as indicated in Chapters 12–14.


_Demand_

Demand flow rates must be specified for each segment and analysis
period. Because analysis of multiple analysis periods is based on
consecutive 15-min periods, the demand flow rates for each period must be
provided. This condition is in addition to the requirements for isolated
segment analyses.

Demand flow rates must be specified for the entering freeway mainline
flow and for each on-ramp and off-ramp within the defined facility. The
following information is needed for each analysis period to determine the
demand flow rate:

   - Demand flow rate (veh/h),

   - Percent single-unit trucks and buses, and

   - Percent tractor-trailer trucks.

For weaving segments, demand flow rates must be identified by
component movement: freeway to freeway, ramp to freeway, freeway to
ramp, and ramp to ramp. Where this level of detail is not available, the
following procedure may be used to estimate the component flows. It is less
desirable, however, since weaving segment performance is sensitive to the
split of demand flows.

   - _Ramp-weave segments:_ Assume that the ramp-to-ramp flow is 0. The
ramp-to-freeway flow is then equal to the on-ramp flow; the freewayto-ramp flow is then equal to the off-ramp flow.

   - _Major weave segments:_ On-ramp flow is apportioned to the two exit
legs (freeway and ramp) in the same proportion as the total flow on
the exit legs (freeway and ramp).


_Geometry_


All geometric features for each segment of the facility must be specified,
including the following:

   - Number of lanes;

   - Average lane width;

   - Right-side lateral clearance;

   - Terrain;

   - Free-flow speed; and

   - Location of merge, diverge, and weaving segments, with all internal
geometry specified, including the number of lanes on ramps and at
ramp–freeway junctions or within weaving segments, lane widths,
existence and length of acceleration or deceleration lanes, distances
between merge and diverge points, and the details of lane
configuration where relevant.

Geometry does not change by analysis period, so this information is
given only once, regardless of the number of analysis periods under study.

Effects of work zones, weather, and incidents can also be included in the
analysis. Section 4 provides an extension of the method for work zone
analysis. Chapter 11 provides default adjustment factors for weather and
incident effects that can be used to calibrate the procedure in Step A-8.


**Step A-4: Balance Demands**

Traffic counts taken at each entrance to and exit from the defined freeway
facility (including the mainline entrance and mainline exit) for each time
interval serve as inputs to the methodology. While entrance counts are
considered to represent the current entrance demands for the freeway facility
(provided there is no queue on the freeway entrance), the exit counts may not
represent the current exit demands for the freeway facility because of
congestion within the defined facility.

For planning applications, estimated traffic demands at each entrance to
and exit from the freeway facility for each time interval serve as inputs to the
methodology. The sum of the input demands must equal the sum of the output
demands in every time interval.


Once the entrance and exit demands are calculated, the demands for each
cell in the time–space domain can be estimated for every analysis period.
The segment demands can be thought of as filtering across the time–space
domain and filling each cell of the time–space matrix.

Estimates of demand are needed when the methodology is applied by
using actual freeway counts. If demand flows are known or can be projected,
they are used directly without modification.

The methodology includes a demand estimation model that converts the
input set of freeway exit 15-min counts to a set of vehicle flows that desire to
exit the freeway in a given 15-min period. This demand may not be the same
as the 15-min exit count because of upstream congestion within the defined
freeway facility.

The procedure sums the freeway entrance demands along the entire
directional freeway facility, including the entering mainline segment, and
compares this sum with the sum of freeway exit counts along the directional
freeway facility, including the departing mainline segment. This procedure is
repeated for each time interval. When sensor data are used to populate the
inputs for this procedure, the total entering and exiting demands in an
analysis period may not be the same if there is congestion internal to the
facility. The ratio of the total facility entrance counts to total facility exit
counts is called the _time interval scale factor_ and should approach 1.00
when the freeway exit counts are, in fact, freeway exit demands.

Scale factors greater than 1.00 indicate increasing levels of congestion
within the freeway facility, with exit counts underestimating the actual
freeway exit demands. To provide an estimate of freeway exit demand, each
freeway exit count is multiplied by the time interval scale factor.

Equation 10-2 and Equation 10-3 summarize this process:


**Equation 10-2:**


**Equation 10-3:**


where

_fTISi_ = time interval scale factor for analysis period _i_,

_VON15ij_ = 15-min entering count for analysis period _i_ and entering
location _j_ (veh),

_VOFF15ij_ = 15-min exit count for analysis period _i_ and exiting location
_j_ (veh), and

_VdOFF15ij_ = adjusted 15-min exit demand for analysis period _i_ and
exiting location _j_ (veh).


Once the entrance and exit demands are determined, the traffic demands
for each section and each analysis period can be calculated. On the time–
space domain, section demands can be viewed as projecting horizontally
across Exhibit 10-10, with each cell containing an estimate of its 15-min
demand.


**Step A-5: Code Base Facility**

This is the first step requiring the use of a computational engine or
software. While individual analysis periods with undersaturated operations
can be evaluated manually with this chapter’s procedure, computations over
multiple analysis periods and computations involving oversaturated segments
and analysis periods require the use of a computational engine. A
computational engine is available as part of Volume 4 of the HCM for
research purposes. Commercial software packages that implement the
method are also available.

Data input needs for the engine or other tools include all items collected
or estimated in the previous steps. Data generally need to be entered for each
segment and each analysis period, which makes this one of the most timeconsuming steps in the analysis.


**Step A-6: Identify Global Parameters**

This step defines global (facilitywide) parameters that are needed for
computation and calibration. While most inputs to the methodology can
change at the segment and analysis period level, two global parameters are
used across the entire spatial and temporal domains:

   - _Jam density_, which is defined as the maximum achievable density in
a segment under congested flow conditions, equivalent to a
theoretical flow rate and segment speed of zero. The jam density
affects the oversaturated speed–flow–density relationship used for
calculations. The default value for jam density is 190 pc/mi/ln.

   - _Queue discharge capacity drop,_ which is defined as a percent
reduction in the prebreakdown capacity following breakdown at an
active bottleneck. The postbreakdown flow rate or queue discharge
rate is defined as the average flow rate during oversaturated
conditions (i.e., during the time interval after breakdown and before
recovery). This factor directly affects the throughput at a bottleneck
and therefore the overall facility performance. The default value for
the queue discharge capacity drop is 7%, on the basis of research ( _7_ ).

Use of these parameters in the oversaturated flow portion of the
methodology and their expected effects on calibration and validation are
described in Chapter 25, Freeway Facilities: Supplemental.


**Step A-7: Compute Segment Capacities**

Segment capacity estimates are determined by the methodologies of
Chapter 12 for basic freeway segments, Chapter 13 for weaving segments,
and Chapter 14 for merge and diverge segments. All estimates of segment
capacity should be carefully reviewed and compared with local knowledge
and available traffic information for the study site, particularly where there
are known bottlenecks.

On-ramp and off-ramp roadway capacities are also determined in this
step with the Chapter 14 methodology. On-ramp demands may exceed onramp capacities, which would limit the traffic demand entering the facility.
Off-ramp demands may exceed off-ramp capacities, which would cause


congestion on the freeway, although that impact is not accounted for in this
methodology.

All capacity results are stated in vehicles per hour under prevailing
roadway and traffic conditions.

The effect of a predetermined ramp-metering plan can be evaluated in
this methodology by overriding the computed ramp roadway capacities. The
capacity of each entrance ramp in each time interval is changed to reflect the
specified ramp-metering rate. This approach not only allows for evaluating a
prescribed ramp-metering plan but also permits the user to improve the
ramp-metering plan through experimentation. The analysis can further
estimate the extent of on-ramp queuing, but the same queue density as the
mainline queue is assumed.

Freeway design improvements can be evaluated with this methodology
by modifying the design features of any portion of the freeway facility. For
example, the effects of adding auxiliary lanes at critical locations and full
lanes over multiple segments can be assessed.


**Step A-8: Calibrate with Adjustment Factors**

Segment capacities can be affected by a number of conditions, some of
which may not normally be accounted for in the segment methodologies of
Chapters 12–14. These reductions include the effects of adverse weather
conditions, other environmental factors, driver population, and incidents.
Adjustments for work zones and lane closures for construction or major
maintenance operations are discussed in Section 4.

This step allows the user to adjust demands, capacities, and free-flow
speeds for the purpose of calibration or to reflect the impacts of weather,
incidents, and work zones. The demand adjustment factor _DAFcal_, capacity
adjustment factor _CAFcal_, and speed adjustment factor _SAFcal_ can be
modified for each segment and each analysis period. The adjustment factors
are used as multipliers for the base demand, capacity, and free-flow speeds
input into the methodology.

It is strongly recommended that _the base run not include any_
_adjustments_, with the three adjustment factors above being used as
calibration tools in one or more subsequent iterations with the intent of


matching field data. CAF and SAF values should always be equal to or less
than 1.0, since they are intended to adjust the base values downwards. If
needed, a higher base value can be used and then calibrated downward with
the CAF and SAF factors. DAFs are primarily used in the context of a
freeway reliability analysis, as discussed in Chapter 11.

An adjusted free-flow speed _FFSadj_ is calculated by multiplying the FFS
by a _SAFcal_ as shown in Equation 10-4:


**Equation 10-4:**


An adjusted capacity _cadj_ is calculated by multiplying the base capacity _c_
by a _CAFcal_ as shown in Equation 10-5:


**Equation 10-5:**


An adjusted demand input volume _vadj_ is calculated by multiplying the
base demand volume _v_ by a _DAFcal_ as shown in Equation 10-6:


**Equation 10-6:**


At lane drops, permanent reductions in capacity occur. These effects are
included in the core methodology, which determines segment capacity on the
basis of the number of lanes in the segment and other prevailing conditions.
However, the method does not account for frictional effects at lane drops,
which may be needed to calibrate the facility operation properly.

Speed and capacity adjustment factors for weather, other environmental
conditions, and incidents are found in Chapter 12, Basic Freeway and
Multilane Highway Segments _._ Adjustments for driver population
characteristics are discussed in Chapter 26; since no default values for


driver population adjustments are presently available, these adjustments need
to be estimated locally. The application of these adjustment factors to
different segment types is described in Chapters 12, 13, and 14 as
applicable.


**Step A-9: Managed Lane Cross-Weave Adjustment**

This step is only required for facilities with managed lanes. It
implements a friction factor for traffic from a general purpose on-ramp that
weaves across the general purpose lanes to get to a managed lane access
point (or vice versa). The cross-weave adjustment factor is conceptually
similar to the CAF used in Step A 8 and is discussed in detail in Section 4.


**Step A-10: Compute Demand-to-Capacity Ratios**

Each cell of the time–space domain now contains an estimate of demand
and capacity. A demand-to-capacity ratio can be calculated for each cell.
The cell values must be carefully reviewed to determine whether all
boundary cells have _vd/c_ ratios of 1.00 or less and to determine whether any
cells in the interior of the time–space domain have _vd/c_ values greater than
1.00.

If any boundary cells have a _vd/c_ ratio greater than 1.00, further analysis
may be significantly flawed:

1. If any cell in the first time interval has a _vd/c_ ratio greater than 1.00,
there may have been oversaturated conditions in earlier time intervals
without transfer of unsatisfied demand into the time–space domain of
the analysis.

2. If any cell in the last time interval has a _vd/c_ ratio greater than 1.00,
the analysis will be incomplete because the unsatisfied demand in the
last time interval cannot be transferred to later time intervals.

3. If any cell in the last downstream segment has a _vd/c_ ratio greater than
1.00, there may be downstream bottlenecks that should be checked
before proceeding with the analysis. If any cell in the first segment
has a _vd/c_ ratio greater than 1.00, oversaturation will extend upstream


of the defined freeway facility, but its effects will not be analyzed
within the time–space domain.

These checks do not guarantee that the boundary cells will not show _vd/c_
ratios greater than 1.00 later in the analysis. If these initial checks reveal
boundary cells with _vd/c_ ratios greater than 1.00, the time–space domain of
the analysis should be adjusted to eliminate the problem.

As the analysis of the time–space domain proceeds, subsequent demand
shifts may cause some boundary cell _vd/c_ ratios to exceed 1.00. In these
cases, the problem should be reformulated or alternative tools applied. Most
alternative tools will have the same problem if the boundary conditions
experience congestion.

Another important check is to observe whether any cell in the interior of
the time–space domain has a _vd/c_ ratio greater than 1.00. There are two
possible outcomes:

1. If all cells have _vd/c_ ratios of 1.00 or less, the entire time–space
domain contains undersaturated flow, and the analysis is greatly
simplified.

2. If any cell in the time–space domain has a _vd/c_ ratio greater than 1.00,
the time–space domain will contain both undersaturated and
oversaturated cells. Analysis of oversaturated conditions is much
more complex because of the interactions between freeway segments
and the shifting of demand in both time and space.

If Case 1 exists, the analysis moves to Step A-11. If Case 2 exists, the
analysis moves to Step A-12.

The _vd/c_ ratio for all on-ramps and off-ramps should also be examined. If
an on-ramp demand exceeds the on-ramp capacity, the ramp demand flow
rates should be adjusted to reflect capacity. Off-ramps generally fail because
of deficiencies at the ramp–street junction. They may be analyzed by
procedures in Chapters 19–22, depending on the type of traffic control used
at the ramp–street junction. These checks are done manually, and inputs to
this methodology must be revised accordingly.


**Steps A-11 and A-12: Compute Undersaturated and**
**Oversaturated Performance Measures**

The analysis begins in the first cell in the upper-left corner of the time–
space domain (the first segment in the first time interval) and continues
downstream along the freeway facility for each segment in the first time
interval. The analysis then returns to the first upstream segment in the second
time interval and continues downstream along the freeway for each segment
in the second time interval. This process continues until all cells in the time–
space domain have been analyzed (refer back to Exhibit 10-10 for an
illustrative example).

As each cell is analyzed in turn, its _vd/c_ ratio is checked. If the _vd/c_ ratio
is 1.00 or less, the cell is not a bottleneck and is able to handle all traffic
demand that wishes to enter. The process is continued in the order noted in
the previous paragraph until a cell with a _vd/c_ ratio greater than 1.00 is
encountered. Such a cell is labeled as a bottleneck. Because the bottleneck
cannot handle a flow greater than its capacity, the following impacts will
occur:

1. The _va/c_ ratio of the bottleneck cell will be exactly 1.00, since the
cell processes a flow rate equal to its capacity.

2. Flow rates for all cells downstream of the bottleneck must be
adjusted downward to reflect the fact that not all the demand flow at
the bottleneck is released. Downstream cells are subject to demand
starvation due to the bottleneck metering effect.

3. The unsatisfied demand at the bottleneck cell must be stored in the
upstream segments. Flow conditions and performance measures in
these upstream cells are affected. Shock wave analysis is applied to
estimate these impacts.

4. The unsatisfied demand stored upstream of the bottleneck cell must
be transferred to the next time interval. The transfer is accomplished
by adding the unsatisfied demand by desired destination to the origin–
destination table of the next time interval.

This four-step process is implemented for each bottleneck encountered,
following the specified sequence of cell analysis. If no bottlenecks are


identified, the entire domain is undersaturated, and the sequence of steps for
oversaturated conditions is not applied.

If a bottleneck is severe, the storage of unsatisfied demand may extend
beyond the upstream boundary of the freeway facility or beyond the last time
interval of the time–space domain. In such cases, the analysis will be flawed,
and the time–space domain should be reconstituted.

After all demand shifts (in the case of one or more oversaturated cells)
are estimated, each cell is analyzed by the methodologies of Chapters 11, 12,
and 13. Facility service and performance measures may then be estimated.


_Step A-11: Undersaturated Conditions_

For undersaturated conditions, the process is straightforward. Because
there are no cells with _vd/c_ ratios greater than 1.00, the flow rate in each cell
_va_ is equal to the demand flow rate _vd_ . Each segment analysis using the
methodologies of Chapters 12–14 will result in estimating a density _D_ and a
space mean speed _S_ .

When the analysis moves from isolated segments to a facility, additional
constraints may be necessary. A maximum-achievable-speed constraint is
imposed to limit the prediction of speeds in segments downstream of a
segment experiencing low speeds. This constraint prevents large speed
fluctuations from segment to segment when the segment methodologies are
directly applied. This process results in some changes in the speeds and
densities predicted by the segment methodologies.

For each time interval, Equation 10-1 is used to estimate the average
density for the defined freeway facility. This result is compared with the
criteria of Exhibit 10-6 to determine the facility LOS for the analysis period.
Each analysis period will have a separate LOS. Although LOS is not
averaged over time intervals, if desired, density can be averaged over time
intervals.


_Step A-12: Oversaturated Conditions_

Once oversaturation is encountered, the methodology changes its
temporal and spatial units of analysis. The spatial units become nodes and
segments, and the temporal unit moves from a time interval of 15 min to


smaller time steps, as recommended in Chapter 25, Freeway Facilities:
Supplemental.

Exhibit 10-13 illustrates the node–segment concept. A node is defined as
the junction of two segments. Since there is a node at the beginning and end
of the freeway facility, there will always be one more node than the number
of segments on the facility.


**Exhibit 10-13: Node–Segment Representation of a Freeway Facility**


The numbering of nodes and segments begins at the upstream end of the
defined freeway facility and moves to the downstream end. The segment
upstream of node _i_ is numbered _i_ - 1, and the downstream segment is
numbered _i_, as shown in Exhibit 10-14.


**Exhibit 10-14: Mainline and Segment Flow at On- and Off-Ramps**


Note: _SF_ = segment flow, _MF_ = mainline flow, _ONRF_ = on-ramp flow, and _OFRF_ = off-ramp
flow.


The oversaturated analysis moves from the first node to each downstream
node for a time step. After the analysis for the first time step is complete, the
same nodal analysis is performed for each subsequent time step.

When oversaturated conditions exist, many flow variables must be
adjusted to reflect the upstream and downstream effects of bottlenecks. These
adjustments are explained in general terms in the sections that follow and are
fully detailed in Chapter 25.


_Flow Fundamentals_

As noted previously, segment flow rates must be calculated for each time
step. They are used to estimate the number of vehicles on each segment at the
end of every time step. The number of vehicles on each segment is used to
track queue accumulation and discharge and to estimate the average segment
density.

The conversion from standard 15-min time intervals to time steps (of
lesser duration) occurs during the first oversaturated interval. Time steps are
then used until the analysis is complete. This transition to time steps is
critical because, at certain points in the methodology, future performance is
estimated from the past performance of an individual variable. The use of
time steps also allows for a more accurate estimation of queues.

Service and other performance measures for oversaturated conditions use
a simplified, linear flow–density relationship, as detailed in Chapter 25.


_Segment Initialization_

To estimate the number of vehicles on each segment for each time step
under oversaturated conditions, the process must begin with the appropriate
number of vehicles in each segment. Determining this number is referred to
as _segment initialization._

A simplified queuing analysis is initially performed to account for the
effects of upstream bottlenecks. The bottlenecks limit the number of vehicles
that can proceed downstream.

To obtain the proper number of vehicles on a given segment, an _expected_
_demand_ is calculated that includes the effects of all upstream segments. The
expected demand represents the flow that would arrive at each segment if all
queues were stacked vertically (i.e., as if the queues had no upstream
impacts). For all segments upstream of a bottleneck, the expected demand
will equal the actual demand.

For the bottleneck segment and all further downstream segments, a
capacity restraint is applied at the bottleneck when expected demand is
computed. From the expected segment demand, the background density can
be obtained for each segment by using the appropriate estimation algorithms
from Chapters 12–14.


_Mainline Flow Calculation_

Flows analyzed in oversaturated conditions are calculated for every time
step and are expressed in vehicles per time step. They are analyzed
separately on the basis of the origin and destination of the flow across the
node. The following flows are defined:

1. The flow from the mainline upstream segment _i_   - 1 to the mainline
downstream segment _i_ is the mainline flow _MF_ .

2. The flow from the mainline to an off-ramp is the off-ramp flow
_OFRF_ .

3. The flow from an on-ramp to the mainline is the on-ramp flow _ONRF_ .

Each of these flows was illustrated in Exhibit 10-14.


_Mainline Input_

The mainline input is the number of vehicles that wish to travel through a
node during the time step. The calculation includes the effects of bottlenecks
upstream of the subject node. These effects include the metering of traffic
during queue accumulation and the presence of additional vehicles during
queue discharge.

The mainline input is calculated by taking the number of vehicles entering
the node upstream of the analysis node, adding on-ramp flows or subtracting
off-ramp flows, and adding the number of unserved vehicles on the upstream
segment. The result is the maximum number of vehicles that desire to enter a
node during a time step.


_Mainline Output_

The mainline output is the maximum number of vehicles that can exit a
node, constrained by downstream bottlenecks or by merging traffic. Different
constraints on the output of a node result in three different types of mainline
outputs (MO1, MO2, and MO3).

   - _Mainline output from ramps (MO1):_ MO1 is the constraint caused
by the flow of vehicles from an on-ramp. The capacity of an on-ramp
flow is shared by two competing flows: flow from the on-ramp and
flow from the mainline. The total flow that can pass the node is


estimated as the minimum of the segment _i_ capacity and the mainline
outputs (MO2 and MO3) calculated in the preceding time step.

   - _Mainline output from segment storage (MO2):_ The output of
mainline flow through a node is also constrained by the growth of
queues on the downstream segment. The presence of a queue limits
the flow into the segment once the queue reaches its upstream end.
The queue position is calculated by shock wave analysis. The MO2
limitation is determined first by calculating the maximum number of
vehicles allowed on a segment at a given queue density. The
maximum flow that can enter a queued segment is the number of
vehicles leaving the segment plus the difference between the
maximum number of vehicles allowed on a segment and the number
of vehicles already on the segment. The queue density is determined
from the linear congested portion of the density–flow relationship
shown in Chapter 25.

   - _Mainline output from front-clearing queue (MO3):_ The final
limitation on exiting mainline flows at a node is caused by frontclearing downstream queues. These queues typically occur when
temporary incidents clear. Two conditions must be satisfied: ( _a_ ) the
segment capacity (minus the on-ramp demand if present) for the
current time interval must be greater than the segment capacity (minus
on-ramp demand) in the preceding time interval, and ( _b_ ) the segment
capacity minus the ramp demand for the current time interval must be
greater than the segment demand in the same time interval. Frontclearing queues do not affect the segment throughput (which is limited
by queue throughput) until the recovery wave has reached the
upstream end of the segment. The shock wave speed is estimated
from the slope of the line connecting the bottleneck throughput and the
segment capacity points.


_Mainline Flow_

The mainline flow across node _i_ is the minimum of the following
variables:

   - Node _i_ mainline input,

   - Node _i_ MO2,


   - Node _i_ MO3,

   - Segment _i_   - 1 capacity, and

   - Segment _i_ capacity.


_Determining On-Ramp Flow_

The on-ramp flow is the minimum of the on-ramp input and output. Ramp
input in a time step is the ramp demand plus any unserved ramp vehicles
from a previous time step.

On-ramp output is limited by the ramp roadway capacity and the rampmetering rate. It is also affected by the volumes on the mainline segments.
The latter is a complex process that depends on the various flow
combinations on the segment, the segment capacity, and the ramp roadway
volumes. Details of the calculations are presented in Chapter 25.


_Determining Off-Ramp Flow_

The off-ramp flow is determined by calculating a diverge percentage on
the basis of the segment and off-ramp demands. The diverge percentage
varies only by time interval and remains constant for vehicles that are
associated with a particular time interval. If there is an upstream queue,
traffic to this off-ramp may be metered. This will cause a decrease in the offramp flow. When vehicles that were metered arrive in the next time interval,
they use the diverge percentage associated with the preceding time interval.
This methodology ensures that all off-ramp vehicles prevented from exiting
during the presence of a bottleneck are appropriately discharged in later time
intervals.


_Determining Segment Flow_

Segment flow is the number of vehicles that flow out of a segment during
the current time step. These vehicles enter the current segment either to the
mainline or to an off-ramp at the current node, as shown in Exhibit 10-13.
The number of vehicles on each segment in the current time step is calculated
on the basis of

   - The number of vehicles that were in the segment in the previous time
step,


   - The number of vehicles that entered the segment in the current time
step, and

   - The number of vehicles that can leave the segment in the current time
step.

Because the number of vehicles that leave a segment must be known, the
number of vehicles on the current segment cannot be determined until the
upstream segment is analyzed.

The number of unserved vehicles stored on a segment is calculated as the
difference between the number of vehicles on the segment and the number of
vehicles that would be on the segment at the background density.


_Determining Segment Service Measures_

In the last time step of a time interval, the segment flows in each time step
are averaged over the time interval, and the service measures for each
segment are calculated. If there were no queues on a particular segment
during the entire time interval, the performance measures are calculated from
Chapters 12, 13, and 14 as appropriate.

If there was a queue on the current segment during the time interval, the
performance measures are calculated in four steps:

1. The average number of vehicles over a time interval is calculated for
each segment.

2. The average segment density is calculated by taking the average
number of vehicles in all time steps (in the time interval) and
dividing it by the segment length.

3. The average speed on the current segment during the current time
interval is calculated as the ratio of segment flow to density.

4. The final segment performance measure is the length of the queue at
the end of the time interval (if one exists), which is calculated by
using shock wave theory.

On-ramp queue lengths can also be calculated. A queue will form on the
on-ramp roadway only if the flow is limited by a meter or by freeway traffic
in the gore area. If the flow is limited by the ramp roadway capacity,
unserved vehicles will be stored on a facility upstream of the ramp roadway,


most likely a surface street. The methodology does not account for this delay.
If the queue is on a ramp roadway, its length is calculated by using the
difference in background and queue densities.


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

This step adjusts the performance of (undersaturated) managed lanes
when the adjacent general purpose lanes operate with a density greater than
35 pc/mi/ln, depending on the separation type between the two lane groups
(i.e., paint, buffer, barrier). This step only applies to facilities with managed
lanes and is discussed in more detail in Section 4.


**Step A-14: Compute Lane Group Performance**

This step computes the performance measure for the length of the facility
for each lane group separately. This step only applies to facilities with
managed lanes and is discussed in more detail in Section 4.


**Step A-15: Compute Freeway Facility Performance Measures by**
**Time Interval**

The previously discussed traffic performance measures can be
aggregated over the length of the defined freeway facility for each analysis
period. Aggregations over the entire time–space domain of the analysis are
also mathematically possible, although LOS is defined only for each 15-min
analysis period.

The performance measures include the computation of queue spillback
under oversaturated conditions. All congestion should be fully contained
within the specified time–space domain. If congestion remains at the end of
the last time interval or if queues spill back beyond the first segment at any
time in the analysis, the analysis returns to Step A-5 and the time–space
domain is expanded accordingly.


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**


In this step, the aggregated methodology results at the section level are
compared with field data or results from another model. Additional details
on criteria for calibration and validation of the facility are provided in
Chapter 25, Freeway Facilities: Supplemental _._ If an acceptable match is not
obtained, the analysis returns to Step A-6 and follows the steps for
calibration adjustments.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**

This final step of the core methodology estimates the LOS for each
segment, lane group, and the overall facility for each analysis period.
Freeway facility LOS is defined for each time interval included in the
analysis. An average density for each time interval, weighted by length of
segments and numbers of lanes in segments, is calculated by using Equation
10-1 and is compared against the criteria of Exhibit 10-6.

Step A-17 concludes the core freeway facility methodology for the
analysis of a single study period analysis. However, the analyst may choose
to continue to perform a reliability analysis or evaluation of ATDM
strategies as described in Chapter 11, Freeway Reliability Analysis.


# **4. EXTENSIONS TO THE METHODOLOGY**

**WORK ZONE ANALYSIS**

This section provides methods for analyzing freeway facilities that
include a work zone. The methodology described in this section is largely
based on results from NCHRP Project 03-107 ( _4, 5_ ). Construction activities
can influence traffic operations on freeway facilities by reducing capacity,
free-flow speed, or both. Changes in one or both also affect the speed–flow
relationship.

Research ( _4, 5_ ) shows that the lane configuration, barrier type, area type,
lateral distance of the work zone from traveled lanes, and lighting conditions
(i.e., daytime or night) can affect the capacity of a work zone. This research
also shows that non–work zone free-flow speed, work zone speed limit, lane
configuration, barrier type, presence of ramps, and lighting conditions can
affect the free-flow speed.


**Spatial and Temporal Limits**

Similar to the freeway facility methodology analysis, the work zone
methodology is limited to 15-min analysis periods as the smallest time unit.
The spatial and temporal limits are consistent with the core facility
methodology.

With many work zones and construction activities being present during
nighttime and off-peak conditions, the analyst may consider temporal
extension of the analysis to include both peak and off-peak conditions.

For example, an analysis may explore feasible temporal extents of
nighttime lane closures. In this case, the analyst may consider a temporal
extent from before the p.m. peak period (before any congestion, say 4 p.m.)
until after the a.m. peak period (after morning congestion has cleared, say 10
a.m.). The analyst may then consider various lane closure scenarios between,


for example, 8 p.m. and 5 a.m. within a total analysis scope covering an 18-h
period from 4 p.m. until 10 a.m. the next morning.


**Limitations of the Methodology**

The work zone analysis methodology has the following limitations:

1. The methodology gives an estimate of capacity and free-flow speed
reductions in work zone conditions. These estimates should only be
used when local data are not available.

2. The methodology should be used with caution on steep upgrades
when a single travel lane is open. In this condition, heavy vehicles
may slow down to crawl speeds, with no opportunity for passenger
cars to pass.

3. The methodology does not account for the impacts of law
enforcement (e.g., police car presence) on free-flow speed and
capacity.

4. The methodology does not model the impacts of different pavement
conditions (e.g., milled surface) on free-flow speed and capacity.

5. The methodology assumes a nominal lane width of 12 ft within
freeway work zones. Users may need to adjust the results for
narrower lane widths.

In addition, all limitations of the core methodology also apply to the
work zone extensions.


**Required Data and Sources**

To determine the impacts of a work zone on basic freeway segment
capacity, the analyst must first specify the lane closure type (e.g., shoulder
closure, three-to-two lane closure), barrier type, area type, lateral distance,
and whether daytime or nighttime operations are considered.

To determine the work zone impacts _on free-flow speed,_ the analyst must
specify the ratio of non–work zone speed limit to work zone speed limit, the
work zone regulatory speed limit, lane closure type, barrier type, day or night
work, and the number of ramps within 3 mi of the center of the work zone.


The variables are defined as follows:

_LCSI_ = lane closure severity index (described below);

_fBr_ = indicator variable for barrier type:

= 0 for concrete and hard barrier separation, and

= 1 for cone, plastic drum, or other soft barrier separation;

_fAT_ = indicator factor for area type:

= 0 for urban areas (i.e., typified by high development densities
or concentrations of population), and

= 1 for rural areas (i.e., areas with widely scattered
development and low housing and employment densities);

_fLAT_ = lateral distance from the edge of travel lane adjacent to the
work zone to the barrier, barricades, or cones (0–12 ft);

_fDN_ = indicator variable for daylight or night:

= 0 for daylight, and

= 1 for night;

_fSr_ = speed ratio (decimal); the ratio of non–work zone speed limit
(before the work zone was established) to work zone speed
limit;

_SLwz_ = work zone speed limit (mi/h); and

_TRD_ = total ramp density along the facility (ramps/mi); for isolated
segment analyses, ramps should be counted 3 mi upstream and 3
mi downstream of the center of the work zone.


The barrier type indicator variable can largely be interpreted as
synonymous with the distinction between short-term and long-term work
zones, with longer-term work zones more likely to be configured with
concrete barriers. In research, the barrier type was found to be more clearly
defined and more readily applied than the distinction between short-term and
long-term work zone effects. For long-term work zones, drivers may benefit
from a learning effect that increases capacities over time, but no conclusive
evidence in this regard was found in the research.


The lane closure configuration in a work zone is expressed as _the ratio_
_of the number of original lanes to the number of lanes present in the work_
_zone_ . For instance, a 3-to-1 lane closure configuration means that three lanes
are normally available, but that two lanes were closed during construction
and only one lane was open. Research indicates that this ratio is effective in
showing the influences of different lane configurations on speed or capacity.

This ratio cannot distinguish a 4-to-2 lane closure configuration from a 2to-1 configuration, since both yield a ratio of 0.5. Field observations ( _5_ ) and
citations in the literature ( _4_ ) both suggest that the per lane capacity of a 2-to1 lane closure is significantly less than that of a 4-to-2 closure, due to fewer
open lanes being available. The lane closure severity index (LCSI)
distinguishes such lane closure configurations. Equation 10-7 shows how the
LCSI is calculated:


**Equation 10-7:**


where

_LCSI_ = lane closure severity index (decimal);

_OR_ = open ratio, the ratio of the number of open lanes during road
work to the total (or normal) number of lanes (decimal); and

_No_ = number of open lanes in the work zone (ln).


The LCSI clearly gives a unique value for different lane closure
configurations, where higher values generally correspond to a more severe
lane closure scenario. This is illustrated in Exhibit 10-15. For severe lane
closures, such as 3-to-1 or 4-to-1 for which the computed LCSI from
Equation 10-7 is greater than 2, the LCSI should be limited to 2.


**Exhibit 10-15: Lane Closure Severity Index Values for Different Lane Closure**
**Configurations**


**Number of Total Lane(s)** **Number of Open Lane(s)** **Open Ratio** **LCSI**

3 3 1.00 0.33


2 2 1.00 0.50
4 3 0.75 0.44
3 2 0.67 0.75
4 2 0.50 1.00
2 1 0.50 2.00


Note: LCSI = lane closure severity index.


In interpreting Exhibit 10-15, it is noted that not all work zones are
associated with lane closure effects. For example, work zones may be
limited to shoulder work only or may feature a lane shift or crossover. This
chapter’s methodology also applies to work zones without lane closures. In
the exhibit, a “2-to-2 work zone” can refer to shoulder closures or
crossovers that do not affect the overall number of travel lanes.


**Adjustments to the Core Methodology**


_Work Zone Capacity and Queue Discharge Rate Model_

Freeway work zone capacity corresponds to the maximum sustainable
flow rate immediately preceding a breakdown. However, measuring the
prebreakdown value in work zones is often not feasible. On the other hand,
queue discharge flow rates can easily be measured by using video cameras
or other data collection tools. Therefore, to arrive at an estimate of
prebreakdown work zone capacity, models to predict queue discharge rate as
a function of work zone configurations and other prevailing conditions are
presented. The queue discharge rate is then converted back to the
corresponding prebreakdown flow rate by using a conversion ratio.

The work zone queue discharge rate is defined as follows:

_The average flow rate immediately downstream of an active bottleneck_
_(following breakdown) measured over a 15-min sampling interval while_
_there is active queuing upstream of the bottleneck._

Equation 10-8 gives a predictive model for freeway work zone queue
discharge rate as a function of the work zone configuration and other
prevailing conditions:


**Equation 10-8:**


where _QDRwz_ is the average 15-min queue discharge rate (pc/h/ln) at the
work zone bottleneck.

As expected, the work zone queue discharge rate is lower at higher LCSI
values, when soft barriers are present, in rural areas, with smaller lateral
clearances, and at night.

The prebreakdown capacity for work zones can be estimated from the
queue discharge flow rate, which is expected to be lower than the
prebreakdown flow rate. Equation 10-9 is used to determine the
prebreakdown capacity:


**Equation 10-9:**


where _cwz_ is the work zone capacity (prebreakdown flow rate) (pc/h/ln), _awz_
is the percentage drop in prebreakdown capacity at the work zone due to
queuing conditions (%), and _QDRwz_ is as defined above.

Research shows an average queue discharge drop of 7% in non–work
zone conditions ( _7_ ) and an average value of 13.4% in freeway work zones
( _4_ ). The underlying research measured prebreakdown capacities as well as
queue discharge rates to estimate the magnitude of _awz_ . When there is little
local information available on _awz_, these values can be used as defaults.

The calculated work zone capacity should not be greater than the non–
work zone capacity, and the result of Equation 10-9 should be capped as
necessary.


_Work Zone Free-Flow Speed Model_

A model for work zone free-flow speed has been developed through
work zone observations during low-flow conditions. The model should only


be used if no local estimates of FFS are available.

Equation 10-10 predicts FFS in freeway work zones on the basis of work
zone configurations and other prevailing conditions:


**Equation 10-10:**


where _FFSwz_ is the work zone free-flow speed (mi/h) and all other variables
are as defined previously. If the speed ratio _fSr_ lies outside the lower- or
upper-bound values shown in Equation 10-10, it should be capped as
needed.

The work zone FFS decreases as the LCSI increases, when soft barriers
are used, at night, and as the ramp density increases. Higher speed ratios
result in higher work zone FFS.

The calculated work zone FFS should not be greater than the non–work
zone FFS, and the result of Equation 10-10 should be capped as needed.


_Work Zone Speed–Flow Model_

Changes in work zone prebreakdown capacity and work zone FFS
influence the overall shape of the speed–flow model in the freeway segments
affected by the work zone. Work zone FFS is determined with Equation 1010, while work zone capacity is determined with Equation 10-8 and Equation
10-9.

Adjustment factors for capacity and FFS are used to reflect the effect of
the work zone on speeds and flows. Equation 10-11 is used to determine the
work zone capacity adjustment factor.


**Equation 10-11:**


where

_CAFwz_ = capacity adjustment factor for a work zone (decimal),

_c_ = basic freeway segment capacity in non–work zone conditions
(pc/h/ln), and

_cwz_ = work zone capacity (prebreakdown flow rate) (pc/h/ln).


Similarly, Equation 10-12 is used to determine the speed adjustment
factor for work zone conditions:


**Equation 10-12:**


where

_SAFwz_ = free-flow speed adjustment factor for work zone (decimal),

_FFS_ = freeway free-flow speed in non–work zone conditions (mi/h),
and

_FFSwz_ = work zone free-flow speed (mi/h).


The calculated capacity and speed adjustment factors are inputs to the
generic basic segment speed–flow relationship described in Chapter 12,
Basic Freeway and Multilane Highway Segments _(_ see Exhibit 12-6).

CAFs and SAFs for work zones should never be greater than 1.0, and the
results of Equation 10-11 and Equation 10-12 should be capped at 1.0
accordingly.


_Adjustments for Other Segment Types_

The queue discharge rate model described above applies to basic
freeway segments. Its estimates should be adjusted further for special
freeway work zone configurations, such as merge segments, diverge
segments, weaving segments, and work zones with directional crossovers.


The relationships presented in this section were derived from fieldcalibrated microsimulation models for the special work zone configurations.

No data were available for the impacts of these configurations on FFS,
so these estimates should be used only when local data are not available.
One exception is the FFS for a directional crossover, which should be
estimated on the basis of the crossover’s geometric design and is
subsequently used as an input to the queue discharge rate estimation.

Details on the adjustments for special work zone configurations are
provided in Chapter 25, Freeway Facilities: Supplemental.


_Special Work Zone Considerations_

Other special considerations apply to work zones with small lateral
clearances, significant heavy vehicle presence, or steep grades. These
impacts are discussed below.


_Minimum Lateral Distance_

Observations have shown that work zones with minimum lateral
clearances can have capacity and free-flow speeds well below the estimates
given by the above models. One such example is shown in Exhibit 10-16. As
seen in the exhibit, lateral clearances on both sides of the road are minimal
and are constrained by concrete barriers. As a result, vehicles have limited
ability to maneuver, which reduces capacity and FFS.


**Exhibit 10-16: Example of Minimum Lateral Clearance in Work Zone**


Note: I-5, Los Angeles, California.


Consequently, work zones with minimum lateral clearance on both sides
are expected to have greatly reduced prebreakdown capacities, queue
discharge flows, and free-flow speeds. Analysts should use caution in
applying the average QDR and FFS models under these conditions.


_Significant Heavy Vehicle Presence on Steep Grades_

The model given previously for work zone queue discharge rate is in
units of passenger cars and therefore incorporates the effects of terrain and
heavy vehicle presence. Headways of heavy vehicles in freeway work zones
are consistent with those on freeway segments without work zones; therefore,
no additional work zone–specific heavy vehicle adjustment factors are
provided.

However, special considerations apply when work zones provide only
one open lane, since vehicles have no ability to pass slower heavy vehicles.
On steep upgrades, heavy vehicles may slow to crawl speeds, as discussed
in Chapter 12. In this case, the traffic following these heavy vehicles will
also travel at crawl speed and the work zone capacity will be lower. An


example of a freeway work zone with only one open lane, a high percentage
of heavy vehicles, and a relatively long upgrade is shown in Exhibit 10-17.


**Exhibit 10-17: Freeway Work Zone with One Open Lane, Trucks, and a Long Upgrade**


Source: Nevada DOT.
Note: I-80, near Elko, Nevada.


**MANAGED LANES ANALYSIS**

This section provides a method for analyzing the operational
performance of facilities with one or more managed lanes, as well as their
interaction with the adjacent general purpose lanes. It does not evaluate the
capacity of a dynamic managed lane, which is determined from the pricing
algorithms. Similarly, it does not provide demand predictions or estimate
changes in demand as a function of different pricing strategies. The
methodology is largely based on the results from NCHRP Project 03-96 ( _1_ ).
Managed lanes may include HOV lanes, HOT lanes, or express toll lanes.

Four types of managed lane (ML) freeway segments are defined in
Chapters 12 through 14: _ML merge and diverge segments_, _ML weaving_
_segments_, _ML access segments,_ and _basic freeway segments_ .

The analysis procedures for general purpose lanes with adjacent
managed lanes build on the core methodology’s segment classification. In


addition, the _lane group_ concept is introduced to allow analysts to assign
separate attributes to managed and general purpose lanes, while retaining a
degree of interaction between the two facilities. The adjacent lane groups
(one general purpose and one managed) are required to have the same
segment length.

The research supporting this chapter found that the composition, FFS,
capacity, and behavior characteristics of managed lane traffic streams are
different from those of general purpose lanes. In addition, interaction effects
between the two lane groups were observed, especially in cases where no
physical barrier separated the managed and general purpose lanes.


**Spatial and Temporal Limits**

Similar to the freeway facility core methodology analysis, the managed
lane methodology is limited to 15-min analysis periods as the smallest time
unit. The spatial and temporal limits are consistent with the core
methodology.


**Limitations of the Methodology**

The managed lane analysis methodology has the following limitations:

1. The methodology cannot address the interaction of merge and diverge
maneuvers occurring at the start and end of the managed lane facility
within the spatial limits of the analysis.

2. The impact of variations in the design of the start and end access
points of the managed lane facilities and the operational impacts from
variations in the design of the termini are not considered.

3. The methodology does not involve demand estimation, especially
demand dynamics due to a pricing component that may be in effect on
the managed lane facility. Demand is considered to be a timedependent input to the method.

4. Managed and general purpose lanes must be jointly assigned in a
feasible lane group. Adjacent managed lane and general purpose
segments are required to have identical lengths and separation type.


When a managed lane is added to an analysis, the general purpose
lane segmentation may change.

5. Queue interactions between general purpose and managed lanes on
the access segments are not explicitly considered in this methodology.
However, the methodology will account for the delay caused by the
presence of queues on access segments.

6. Multiple overlapping breakdowns or bottlenecks on either the general
purpose or the managed lanes are not analyzed and cannot be fully
evaluated by the managed lane methodology. Alternative tools may be
more appropriate for specific applications beyond the capabilities of
the methodology.

7. Spatial, temporal, modal, and total demand responses to traffic
management strategies are not automatically incorporated into the
managed lane methodology. On viewing the facility traffic
performance results, the analyst can modify the input demand
manually to analyze the effect of user-demand responses and traffic
growth. The accuracy of the results depends on the accuracy of the
estimated user-demand responses.

8. The results should be viewed cautiously if the _d/c_ ratio is greater than
1.00 for one or more freeway segments during the first or last
analysis period or for the first freeway segment in any analysis
period.

9. The method does not address conditions in which managed lane offramp capacity limitations result in queues that extend onto the
managed lanes or affect the behavior of managed lane off-ramp
vehicles.

In addition, all limitations of the core methodology apply equally to the
managed lane extensions. Because this chapter’s methodology incorporates
the methodologies for basic, weaving, merging, and diverging freeway
segments for both managed and general purpose lanes, the limitations of
those procedures apply here.


**Required Data and Sources**


For a typical operational analysis, the analyst must specify demand
volumes, roadway geometric information (including number of lanes, lane
width, right-side lateral clearance, and total ramp density), percent heavy
vehicles, peak hour factors, terrain, and capacity and speed calibration
factors, similar to what is required for a general purpose freeway facility
analysis. The only difference is that this information must be specified
separately for the managed and general purpose lane groups. In addition, the
type of separation provided between the managed and general purpose lanes
must be specified.


**Adjustments to the Base Methodology**


_Lane Group Concept_

To capture the interaction effects between the managed and general
purpose lanes while allowing for varying demand, capacity, and speed
inputs, the concept of _lane groups_ is introduced for freeway facilities with
managed lanes. By adopting the lane group concept, an analyst can define
separate attributes for parallel managed lane and general purpose facilities
while retaining the ability to model the interaction between the two facilities.

Each segment of a freeway facility is represented as having either one or
two lane groups, depending on whether a concurrent managed lane segment is
present. Input variables such as geometric characteristics (e.g., number of
lanes), traffic performance attributes (e.g., FFS, capacity), and traffic
demands must be entered separately for each lane group. The methodology is
then applied to assess the operational performance of each lane group, with
consideration given to the empirically derived interaction effects between the
two lane groups.

The following principles apply:

   - A freeway general purpose segment with a parallel managed lane
segment is considered as two adjacent lane groups.

   - Adjacent lane groups (one general purpose and one managed lane
segment) must have identical segment lengths.

   - Adjacent lane groups can be of different segment types. For example,
a basic managed lane segment may be concurrent with a general


purpose diverge segment (see Exhibit 10-18 illustrating this case).

   - Adjacent lane groups may have different geometric characteristics,
including number of lanes, lane widths, and shoulder clearance.

   - Adjacent lane groups may have unique operational attributes,
including FFS, segment capacity, or various capacity- or speedreducing factors.

   - Adjacent lane groups may have unique traffic demand parameters,
which are entered by the user and obtained through an external
process. This chapter’s operational methodology does not predict the
split in demand between the managed and general purpose lanes.

   - The operational performance of adjacent managed and general
purpose lane groups is interdependent, in that congestion in one lane
group may have a frictional effect on operations in the adjacent lane
group. This frictional effect was empirically derived, can be usercalibrated, and is sensitive to the type of physical separation between
lane groups (i.e., striping, buffer, barrier).

Oversaturated managed lane facilities are relatively rare in practice,
since one of the underlying principles for managed lane operations
(especially for tolled facilities) is to ensure that managed lane traffic density
is below the critical density even in peak periods, which in turn guarantees
satisfactory service to managed lane customers. However, congestion on
managed and general purpose lanes can and should be considered by the
method, because many facilities operate during peak periods, and especially
in view of nonrecurring congestion effects (e.g., weather, incidents). Chapter
25 provides details on evaluating oversaturated managed and general
purpose lanes.


_Segmentation Considerations_

To preserve the lane group concept, the segmentation is performed
slightly differently from that for a freeway facility consisting only of general
purpose lanes. An example is illustrated in Exhibit 10-18. In the absence of a
parallel managed lane facility, the general purpose segment in the exhibit
would be treated as one four-lane weaving segment with adjacent basic
segments. However, because segmentation also needs to consider the
managed lane segment types, and because adjacent lane groups need to be of


equal length, the segmentation of the general purpose lane group is as
follows: merge (Segment 1), basic (Segment 2), and diverge (Segment 3).
The corresponding managed lane segments are categorized as ML diverge,
ML basic, and ML basic, respectively.


**Exhibit 10-18: Graphical Illustration of the Managed Lane Segmentation Method**


This example illustrates that the analyst may need to make compromises
in the segmentation process when a general purpose lane with an adjacent
managed lane is analyzed. In this case, evaluation of the general purpose
lanes in isolation is also recommended to explore whether their performance
changes significantly in moving from one (long) weaving segment to three
separate segments. If substantial differences exist, the analyst should use
capacity and speed adjustment factors (CAFs and SAFs) to calibrate the
performance of these three segments and match the results to those of a
general purpose–only analysis.


_Cross-Weave Friction Effect_

Where managed lanes have intermittent at-grade access from the general
purpose lanes, a cross-weave movement may be created as vehicles entering
the general purpose facility have to cross multiple lanes to reach the ML
access segment. The ML access segment, in turn, is analyzed as a weaving
segment to capture its friction. However, the cross-weave friction factor is
applied to the general purpose segment(s) upstream of the actual access
point. Exhibit 10-19 illustrates this cross-weave situation.


**Exhibit 10-19: Cross-Weave Movement Associated with Managed Lane Access and**
**Egress**


Note: ML = managed lane, GP = general purpose.


Exhibit 10-19 illustrates a freeway facility consisting of a managed lane
and three general purpose lanes. Where a general purpose merge is near an
ML access segment, on-ramp vehicles destined for the managed lane must
cross all of the general purpose freeway lanes in the distance _Lcw-min_ . The
cross-weave demand can cause a reduction in the capacity of the general
purpose lanes, which must be considered. While not shown, the same effect
exists when an off-ramp is near the ML access segment, with the distance
_Lcw-min_ measured from the end of the access segment to the off-ramp junction
point.

This effect is different from the weaving turbulence that occurs within the
ML access segment, as vehicles entering and exiting from the managed lane
cross paths within the distance _Lcw-max_ - _Lcw-min_ .

In estimating general purpose segment capacity, the cross-weave
adjustment should be taken into account to quantify the reduction in general
purpose segment capacity as a result of significant managed lane crossweave flows. The adjustment should be applied where there is intermittent
access to the managed lane over an access segment. A comprehensive
methodology is provided in Chapter 13, Freeway Weaving Segments, to
account for cross-weave capacity reduction on the general purpose lanes.


_Adjacent Friction Effect_

The adjacent friction effect applies when the general purpose lane group
operates at densities above a specified threshold. Research has shown that


managed lane operations are affected by these high general purpose lane
densities in cases where no physical separation exists between the two
facilities. For physically separated managed lanes, no adjacent friction effect
applies.

For managed lanes without physical separation, a friction-constrained
speed prediction model is used to estimate managed lane speeds. When the
general purpose lanes operate below the specified density threshold, the nonfriction-based speed prediction model is used. This factor is applied to both
Continuous Access and Buffer 1 basic managed lane segments. Additional
discussion of this effect is provided in Chapter 12.


_Computational Steps_

The computational steps for a managed lane analysis are largely
consistent with the analysis of general purpose lanes. Several additional
steps apply, which were highlighted in Exhibit 10-8 and described in Section
3. Specifically, the four unique computational steps for the managed lane
extension are as follows:

   - Step A-9: Managed Lane Cross-Weave Adjustment,

   - Step A-13: Apply Managed Lane Adjacent Friction Factor,

   - Step A-14: Compute Lane Group Performance, and

   - Step A-17: Estimate LOS and Report Performance Measures for Lane
Groups and Facility.

Of these four steps, only the first has to be applied manually by the
analyst. The other three are performed automatically by a computational
engine.


**ACTIVE TRAFFIC AND DEMAND MANAGEMENT**

The evaluation of ATDM strategies is described in detail in Chapter 11,
Freeway Reliability Analysis. In that chapter, the effects of ATDM strategies
such as ramp metering and hard-shoulder running are described in the context
of a whole-year reliability analysis that covers a range of conditions.
Chapter 11’s methodology is the recommended way for evaluating ATDM
strategies as part of a whole-year analysis.


However, an analyst may also be interested in evaluating the effects of a
specific ATDM strategy on a single “representative” day (study period).
Similar to the 1-day work zone analysis extension discussed above, a single
study period ATDM analysis may provide insights into the relative effects of
various strategies, such as when ATDM investments are compared with
geometric improvements on the facility.

Chapter 37, ATDM: Supplemental, provides an overview of different
ATDM strategies and guidance on their expected effects on facility
performance. The analyst may use the available calibration metrics for
freeway facilities, including capacity, speed, and demand adjustment factors
(CAFs, SAFs, and DAFs) to estimate the effects of those strategies on the
facility. The following list provides examples of other types of strategy
assessments that can be performed by using this chapter’s methodology:

1. A growth factor effect can be added to evaluate traffic performance
when traffic demands are higher or lower than the demand calculated
from the traffic counts. This parameter would be used to undertake a
sensitivity analysis of the effect of demand on freeway performance
and to evaluate future scenarios. In these cases, all cell demand
estimates are multiplied by the growth factor parameter.

2. The effect of a predetermined ramp-metering plan can be evaluated
by modifying the ramp roadway capacities. The capacity of each
entrance ramp in each time interval is changed to the desired metering
rate. This feature permits evaluation of a predetermined rampmetering plan and experimentation to obtain an improved rampmetering plan.

3. Freeway design improvements can be evaluated with this
methodology by modifying the design features of any portion of the
freeway facility. For example, the effect of adding an auxiliary lane at
a critical location or of adding merging or diverging lanes can be
assessed.

4. Reduced-capacity situations can be investigated. The capacity in any
cell or cells of the time–space domain can be reduced to represent
situations such as construction and maintenance activities, adverse
weather, and traffic accidents and vehicle breakdowns.


5. User-demand responses, such as spatial, temporal, modal, and total
demand responses caused by a traffic management strategy, are not
automatically incorporated into the methodology. On viewing the new
freeway traffic performance results, the user can modify the demand
input manually to evaluate the effect of anticipated demand responses.


# **5. APPLICATIONS**

Specific computational steps for the freeway facility methodology were
conceptually discussed and presented in this chapter’s methodology section.
Computational details are provided in Chapter 25, Freeway Facilities:
Supplemental.

This chapter’s methodology is sufficiently complex to require the use of
software for its application. Even for fully undersaturated analyses, the
number and complexity of computations make manual analysis of a case
difficult and extremely time-consuming. Oversaturated analyses are
considerably more complex, and manual solutions are impractical. A
computational engine and accompanying user’s guide are available in Volume
4 for research purposes but should not be used for commercial applications.


**EXAMPLE PROBLEMS**

Chapter 25, Freeway Facilities: Supplemental, provides six example
problems that illustrate the steps in applying the core methodology to a
freeway facility under a variety of conditions. Other examples illustrate the
work zone and managed lane extensions, as well as the freeway facility
planning methodology. Exhibit 10-20 shows the list of example problems.


**Exhibit 10-20: List of Example Problems**


**Example**
**Problem** **Description** **Application**

Operational
1 Evaluation of an undersaturated facility
analysis

Operational
2 Evaluation of an oversaturated facility
analysis

Operational
3 Capacity improvements to an oversaturated facility
analysis

Evaluation of an undersaturated facility with work Operational
4
zone analysis
5 Evaluation of an oversaturated facility with managed Operational
lanes analysis


6 Planning-level evaluation of a freeway facility Planning analysis


**RELATED CONTENT IN THE HCMAG**

The _Highway Capacity Manual Applications Guide_ (HCMAG) _,_
accessible through the online HCM Volume 4, provides guidance on applying
the HCM for freeway facility analyses. Case Study 4 goes through the
process of identifying the goals, objectives, and analysis tools for
investigating LOS on New York State Route 7, a 3-mi route north of Albany.
The case study applies the analysis tools to assess the performance of the
route, to identify areas that are deficient, and to investigate alternatives for
correcting the deficiencies.

This case study includes the following problems related to basic freeway
segments:

1. Problem 4: Analysis of a freeway facility

a. Subproblem 4a: Separation of Alternate Route 7 for HCM
analysis

b. Subproblem 4b: Study of off-peak periods

c. Subproblem 4c: What is the operational performance of
Alternate Route 7 during the peak period?

Although the HCMAG was based on the HCM2000’s procedures and
chapter organization, the general thought process described in its case studies
continues to be applicable to current HCM methods.


**EXAMPLE RESULTS**

This section presents the results of applying this chapter’s methodology
in typical situations. Analysts can use the illustrative results presented in this
section to observe the sensitivity of output performance measures to various
inputs, as well as to help evaluate whether their results are reasonable. The
exhibits in this section are not intended to substitute for an actual analysis
and are deliberately provided in a format large enough to depict general
trends in the results but not large enough to pull out specific results.


Total travel time on a freeway facility is sensitive to a number of input
variables, including the prevailing FFS, demand levels, segment capacity,
percentage drop in queue discharge rate, and demand-to-capacity ratio.
Exhibit 10-21 illustrates the resulting facility-level travel time for values of
FFS ranging from 55 to 75 mi/h for an example 6-mi-long facility (Example
Problem 1 in Chapter 25). As apparent from the exhibit, an increase in the
freeway facility FFS yields a reduction in the travel time. This result is due
to the close association between capacity and FFS, with higher FFS values
generating higher capacities and consequently lower travel times.


**Exhibit 10-21: Facility Travel Time Sensitivity to Free-Flow Speed**


Demand levels and the capacities of different segments along a freeway
facility also influence total travel time. An overall increase in the demand
level is expected to increase facility travel time, while an overall increase in
segment capacity is expected to reduce travel time. Furthermore, an overall
increase in the demand-to-capacity ratio is expected to increase travel time.

Exhibit 10-22 illustrates facility travel time sensitivity to changes in the
demand-to-capacity ratio of the critical segment of a freeway facility.
Specifically, the demand-to-capacity ratio is increased from 0.65 to as high
as 1.4 on the last segment of Example Problem 1 in Chapter 25.


**Exhibit 10-22: Facility Travel Time Sensitivity to** _**d/c**_ **Ratio on Critical Segment**


As apparent from the exhibit, increasing the demand-to-capacity ratio
results in a gradual increase in facility travel time in undersaturated
conditions; however, when demand exceeds the capacity ( _d/c_ - 1.0), travel
time increases at a higher rate with an increase in the demand-to-capacity
ratio.

A change in the percentage drop in capacity, modeling the effect of
postbreakdown queue discharge rate, is also expected to influence travel
time on a freeway facility. A larger drop in the queue discharge rate yields a
longer travel time across the facility, as shown in Exhibit 10-23. Example
Problem 2 in Chapter 25 was used to generate this exhibit. This result occurs
because higher capacity drops mean that when oversaturation occurs, queues
will build up faster and recover more slowly as the queue discharge rate is
lowered.


**Exhibit 10-23: Facility Travel Time Sensitivity to Percentage Drop in Queue Discharge**
**Rate**


**PLANNING, PRELIMINARY ENGINEERING, AND DESIGN**
**ANALYSIS**

The operational methodology for freeway facilities cannot be readily
adapted to planning, preliminary engineering, and design applications
because of the amount of data required and the method’s computational
complexity. However, a separate planning methodology is available for
evaluating a freeway facility in a planning context. The methodology is based
on national research ( _11_ ) and is calibrated to approximate the results of an
operational analysis, but with reduced input needs and computational burden.
The method is introduced below and described in more detail in Chapter 25:
Freeway Facilities: Supplemental.


**Service Volume Tables**

The service volume tables provided in Chapter 12 for basic freeway
segments can be used to obtain a quick planning-level estimate of the service
volumes that can be supported on a freeway. These tables may be applied for
general evaluations of a number of freeway facilities in a specified region.
They should not be used for directly evaluating a specific freeway facility or
for developing detailed facility improvement plans. A full operational


analysis would normally be applied to any freeway facility identified as
potentially needing improvement, with the planning methodology providing
an alternative with reduced data input needs and computational time.


**Segment-Based Planning Applications**

The segment procedures described in Chapters 12, 13, and 14 can also
be used in preliminary engineering and design applications of the
methodology. Various geometric scenarios can be evaluated and compared by
using a travel demand matrix and applying the facility methodology to each
scenario’s segment results.


**Freeway Facility Planning Method**

For planning applications, a simplified planning-level methodology may
be desirable ( _11_ ). The approach is based on and compatible with this
chapter’s operational methodology, but the planning method is specifically
constructed to minimize input data requirements. The planning method covers
both undersaturated and oversaturated flow conditions and produces
estimates of travel time, speed, density, and level of service. The method is
based on the use of sections rather than segments, with a section being
defined as the distance between two ramp gore points. Section breaks also
occur when lanes are added or dropped. The underlying methodology relies
on developing a relationship between the delay rate per unit distance on a
basic freeway segment and the segment’s demand-to-capacity ratio.

For weaving sections, the method applies capacity adjustment factors on
the basis of the volume ratio and segment length. With these factors, a
weaving section’s demand-to-capacity ratio is adjusted and the segment is
then treated similarly to a basic freeway segment.

For ramp sections with merge or diverge segments, or both, the
methodology estimates the segment capacity on the basis of the demand level,
free-flow speed, and space mean speed. Capacity adjustment factors are then
calculated for these sections, and their demand-to-capacity ratios are
adjusted accordingly.

The methodology first estimates demand-to-capacity ratios for each
section. In oversaturated conditions, the number of vehicles queued on a


section in one analysis period is added to its demand in the next analysis
period, and demand-to-capacity ratios are adjusted accordingly.

The freeway facility planning method is discussed in detail in Chapter
25, Freeway Facilities: Supplemental.


**USE OF ALTERNATIVE TOOLS**

General guidance for the use of alternative traffic analysis tools for
capacity and LOS analysis is provided in Chapter 6, HCM and Alternative
Analysis Tools. This section contains specific guidance for applying
alternative tools to the analysis of freeway facilities. Additional information
on this topic may be found in Chapter 25, Freeway Facilities: Supplemental.


**Strengths of the HCM Procedure Compared with Alternative**
**Tools**

This chapter’s procedures were based on extensive research supported
by a significant quantity of field data. They have evolved over a number of
years and represent a consensus of experts. Specific strengths of the HCM
freeway facilities procedures include the following:

   - They provide more detailed algorithms for considering geometric
elements of the facility (such as lane and shoulder width).

   - They provide capacity estimates for each segment of the facility,
which simulation tools do not provide directly (and in some cases
may require as an input).

   - The capacity can be explicitly adjusted to account for weather
conditions, lighting conditions, work zone setup and activity, and
incidents.

   - The calculation of key performance measures, such as speed and
density, is transparent. Simulation tools often use statistics
accumulated over the simulation period to derive various link- or
time-period-specific results, and the derivation of these results may
not be obvious. Thus, the user of a simulation tool must know exactly
which measure is being reported (e.g., space mean speed versus time


mean speed). Furthermore, simulation tools may apply these
measures in ways different from the HCM to arrive at other measures.


**Limitations of the HCM Procedures That Might Be Addressed**
**with Alternative Tools**

Freeway facilities can be analyzed with a variety of stochastic and
deterministic simulation tools. These tools can be useful in analyzing the
extent of congestion when there are failures within the simulated facility
range and when interaction with other freeway segments and other facilities
is present.

Exhibit 10-24 provides a list of the limitations stated earlier in this
chapter, along with their potential for improved treatment by alternative
tools.


**Exhibit 10-24: Limitations of the HCM Freeway Facilities Analysis Procedure**







|Limitation|Potential for Improved Treatment with Alternative Tools|
|---|---|
|Changes in travel<br>time caused by<br>vehicles using<br>alternate routes|Modeled explicitly by dynamic traffic assignment tools|
|Multiple overlapping<br>bottlenecks|Modeled explicitly by simulation tools|
|User-demand<br>responses (spatial,<br>temporal, modal)|Modeled explicitly by dynamic traffic assignment tools|
|Systemwide<br>oversaturated flow<br>conditions|Modeled explicitly by simulation tools|
|First/last time interval<br>or first/last segment<br>demand-to-capacity<br>ratio > 1.0|Modeled explicitly by simulation tools, except that a simulation<br>analysis may also be inaccurate if it does not fully account for a<br>downstream bottleneck that causes congestion in the last segment<br>during the last analysis period|
|Interaction between<br>managed lanes and<br>mixed-flow lanes|Modeled explicitly by some simulation tools|


**Additional Features and Performance Measures Available from**
**Alternative Tools**


This chapter provides a methodology for estimating the following
performance measures for individual segments along a freeway facility and
for the entire facility, given each segment’s traffic demand and
characteristics:

   - Travel time,

   - Free-flow travel time,

   - Traffic delay,

   - Vehicle miles of travel,

   - Person miles of travel,

   - Speed, and

   - Density (segment only).

Alternative tools can offer additional performance measures, such as
queue lengths, fuel consumption, vehicle emissions, operating costs, and
vehicle acceleration and deceleration rates. As with other procedural
chapters in the HCM, simulation outputs—especially graphics-based outputs
—may provide details on point problems that might go unnoticed with a
macroscopic analysis.


**Development of HCM-Compatible Performance Measures Using**
**Alternative Tools**

LOS for all types of freeway segments is estimated by the density of
traffic (pc/mi/ln) on each segment. The guidance provided in Chapter 11,
Basic Freeway Segments, for developing compatible density estimates
applies to freeway facilities as well.

With the exception of free-flow travel time, the additional performance
measures listed above that are produced by the procedures in this chapter are
also produced by typical simulation tools. For the most part, the definitions
are compatible, and, subject to the precautions and calibration requirements
that follow, the performance measures from alternative tools may be
considered equivalent to those produced by the procedures in this chapter.


**Conceptual Differences Between the HCM and Simulation**
**Modeling That Preclude Direct Comparison of Results**

To determine when simulation of a freeway facility may be more
appropriate than an HCM analysis, the fundamental differences between the
two approaches must be understood. The HCM and simulation analysis
approaches are reviewed in the following subsections.


_HCM Approach_

The HCM analysis procedure uses one of two approaches—one for
undersaturated conditions and one for oversaturated conditions. For the
former—that is, _vd/c_ is less than 1.0 for all segments and analysis periods—
the approach is generally disaggregate. In other words, the facility is
subdivided into segments corresponding to basic freeway, weaving, and
merge or diverge segments, and the LOS results are reported for individual
segments on the basis of the analysis procedures of Chapters 12–14. LOS
results are aggregated for the facility as a whole in each analysis period.

For oversaturated conditions, the facility is analyzed in a different
manner. First, the facility is considered in its entirety rather than at the
individual segment level. Second, the analysis time interval, typically 15
min, is subdivided into time steps of 15 s. This approach is necessary so that
flows can be reduced to capacity levels at bottleneck locations and queues
can be tracked in space and time. The average density of an oversaturated
segment is calculated by dividing the average number of vehicles in the
segment across these time steps by the segment length. The average segment
speed is calculated by dividing the average segment flow rate by the average
segment density. Facilitywide performance measures are calculated by
aggregating segment performance measures across space and time, as
outlined in Chapter 25. A LOS for the facility is assigned on the basis of
density for each time interval.

When the oversaturation analysis procedure is applied, if any segment is
undersaturated for an entire time interval, its performance measures are
calculated according to the appropriate procedure in Chapters 12–14.


_Simulation Approach_


Simulation tools model the facility in its entirety and from that
perspective have some similarity to the oversaturated analysis approach of
the HCM. Microscopic simulation tools operate similarly under saturated
and undersaturated conditions. They track each vehicle through time and
space and generally handle the accumulation and queuing of vehicles in
saturated conditions in a realistic manner. Macroscopic simulation tools vary
in their treatment of saturated conditions. Some tools do not handle
oversaturated conditions at all; others may queue vehicles in the vertical
rather than the horizontal dimension. These tools may still provide
reasonably accurate results under slightly oversaturated conditions, but the
results will clearly be invalid for heavily congested conditions.

The treatment of oversaturated conditions is a fundamental issue that must
be understood in considering whether to apply simulation in lieu of the HCM
for analysis of congested conditions. A review of simulation modeling
approaches is beyond the scope of this document. More detailed information
on the topic may be found in the Technical Reference Library in Volume 4.


**Adjustment of Simulation Parameters to the HCM Results**

Some calibration is generally required before an alternative tool can be
used effectively to supplement or replace the HCM procedure. The following
subsections discuss key variables that should be checked for consistency
with the HCM procedure values.


_Capacity_

In the HCM, prebreakdown capacity is a function of the specified or
computed free-flow speed (which can be adjusted by lane width, shoulder
width, and ramp density) and of capacity adjustment factors that account for
local conditions, driver population effects, weather, incidents, and work
zones. In a simulation tool, capacity is typically a function of the specified
minimum vehicle entry headway (into the facility) and car-following
parameters (if the discussion pertains to microscopic simulation).

In macroscopic simulation tools, capacity is generally an input. For this
situation, matching the simulation capacity to the HCM capacity is
straightforward. However, microscopic simulation tools do not have an
explicit capacity input. Most microscopic tools provide an _input that affects_


the minimum separation for the generation of vehicles into the system.
Specifying a value of 1.5 s for this input will result in a maximum vehicle
entry rate of 2,400 (3,600/1.5) veh/h/ln. Once vehicles enter the system,
vehicle headways are governed by the car-following and gap acceptance
models. In view of other factors and model constraints, the maximum
throughput on any one segment may not reach this value. Consequently, some
experimenting is usually necessary to find the right minimum entry separation
value to achieve a capacity value comparable with that in the HCM. Again,
the analyst needs to be mindful of the units being used for capacity in making
comparisons.

The other issue to be aware of is that, while geometric factors such as
lane and shoulder width affect the free-flow speed (which in turn affects
capacity) in the HCM procedure, some simulation tools do not account for
these effects, or they may account for other factors, such as horizontal
curvature, that the HCM procedure does not consider.


_Lane Distribution_

In the HCM procedure, there is an implicit assumption that, for any given
vehicle demand, the vehicles are evenly distributed across all lanes of a
basic freeway segment. For merge and diverge segments, the HCM
procedure includes calculations to determine how vehicles are distributed
across lanes as a result of merging or diverging movements. For weaving
segments, there is not an explicit determination of flow rates in particular
lanes, but consideration of weaving and nonweaving flows and the number of
lanes available for each is an essential element of the analysis procedure.

In simulation tools, the distribution of vehicles across lanes is typically
specified only for the entry point of the network. Once vehicles have entered
the network, they are distributed across lanes according to car-following and
lane-changing logic. This input value should reflect field data if they are
available. If field data indicate an imbalance of flows across lanes, a
difference between the HCM and simulation results may ensue. If field data
are not available, specifying an even distribution of traffic across all lanes is
probably reasonable for networks that begin with a long basic segment. If
there is a ramp junction within a short distance downstream of the entry point
of the network, setting the lane distribution values to be consistent with those
from Chapter 14 of the HCM will likely yield more consistent results.


_Traffic Stream Composition_

The HCM deals with the presence of non–passenger car vehicles in the
traffic stream by applying passenger car equivalent values. These values are
based on the percentage of single-unit trucks, buses, and tractor-trailers in the
traffic stream, as well as the type of terrain (grade profile and its length). The
values also depend on the relative heavy vehicle fleet mix between singleunit trucks (including buses and recreational vehicles) and tractor-trailer
trucks. Thus, the traffic stream is converted into some equivalent number of
passenger cars only, and the analysis results are based on flow rates in these
units.

Simulation tools deal with the traffic stream composition just as it is
specified; that is, the specific percentages of each vehicle type are generated
and moved through the system according to their specific vehicle attributes
(e.g., acceleration and deceleration capabilities). Thus, simulation,
particularly microscopic simulation, results likely better reflect the effects of
non–passenger car vehicles on the traffic stream. Although in some instances
the HCM’s passenger car equivalent values were developed from simulation
data, simplifying assumptions made to implement them in an analytical
procedure result in some loss of fidelity in the treatment of different vehicle
types.

In addition, HCM procedures do not explicitly account for differences in
driver types. Microscopic simulation tools explicitly provide for a range of
driver types and allow a number of factors related to driver type to be
modified (e.g., FFS, gap acceptance threshold). However, the empirical data
supporting some HCM procedures include the effects of the various driver
types present in traffic streams.


_Free-Flow Speed_

In the HCM, FFS is either measured in the field or estimated with
calibrated predictive algorithms. In simulation, FFS is almost always an
input value. Where field measurements are not available, simulation users
may wish to use the HCM predictive algorithms to estimate FFS.


**Step-by-Step Recommendations for Applying Alternative Tools**


General guidance for applying alternative tools is provided in Chapter 6,
HCM and Alternative Analysis Tools. The chapters that cover specific types
of freeway segments offer more detailed step-by-step guidance specific to
those segments. All the segment-specific guidance applies to freeway
facilities, which are configured as combinations of different segments.

The first step is to determine whether the facility can be analyzed
satisfactorily by the procedures described in this chapter. If the facility
contains geometric or operational elements beyond the scope of these
procedures, an alternative tool should be selected. The steps involved in the
application will depend on the reason(s) for choosing an alternative tool. In
some cases, the step-by-step segment guidance will cover the situation
adequately. In more complex cases (e.g., those involving integrated analysis
of a freeway corridor), more comprehensive guidance from one or more
documents in the Technical Reference Library in Volume 4 may be needed.


**Sample Calculations Illustrating Alternative Tool Applications**

The limitations of this chapter’s procedures are mainly related to the lack
of a comprehensive treatment of the interaction between segments and
facilities and between facilities, for example a freeway and parallel surface
street arterial forming a corridor. Many of these limitations can be addressed
by simulation tools, which generally take a more integrated approach to the
analysis of complex networks of freeways, ramps, and surface street
facilities. Supplemental examples illustrating interactions between segments
are presented in Chapter 26, Freeway and Highway Segments: Supplemental,
and Chapter 34, Interchange Ramp Terminals: Supplemental. A
comprehensive example of the application of simulation tools to a major
freeway reconstruction project is presented as Case Study 6 in the HCM
Applications Guide located in Volume 4.


# **6. REFERENCES**

1. Wang, Y., X. Liu, N. Rouphail, B. Schroeder, Y. Yin, and L. Bloomberg.
_NCHRP Web-Only Document 191: Analysis of Managed Lanes on_
_Freeway Facilities._ Transportation Research Board of the National
Academies, Washington, D.C., Aug. 2012.
http://www.trb.org/Publications/Blurbs/168255.aspx

2. Liu, X., B. J. Schroeder, T. Thomson, Y. Wang, N. M. Rouphail, and Y.
Yin. Analysis of Operational Interactions Between Freeway Managed
Lanes and Parallel, General Purpose Lanes. In _Transportation_
_Research Record: Journal of the Transportation Research Board, No._
_2262_, Transportation Research Board of the National Academies,
Washington, D.C., 2011, pp. 62–73.

3. Schroeder, B. J., S. Aghdashi, N. M. Rouphail, X. C. Liu, and Y. Wang.
Deterministic Approach to Managed Lane Analysis on Freeways in
Context of _Highway Capacity Manual_ . In _Transportation Research_
_Record: Journal of the Transportation Research Board, No. 2286_,
Transportation Research Board of the National Academies, Washington,
D.C., 2012, pp. 122–132.

4. Yeom, C., A. Hajbabaie, B. J. Schroeder, C. Vaughan, X. Xuan, and N.
M. Rouphail. Innovative Work Zone Capacity Models from Nationwide
Field and Archival Sources. In _Transportation Research Record:_
_Journal of the Transportation Research Board, No. 2485_,
Transportation Research Board, Washington, D.C., 2015, pp. 51–60.

5. Hajbabaie, A., C. Yeom, N. M. Rouphail, W. Rasdorf, and B. J.
Schroeder. Freeway Work Zone Free-Flow Speed Prediction from
Multi-State Sensor Data. Presented at 94th Annual Meeting of the
Transportation Research Board, Washington, D.C., 2015.

6. Elefteriadou, L., H. Xu, and L. Xe. _Travel Time Reliability Models._
Florida Department of Transportation, Tallahassee, Aug. 2008.


7. Hu, J., B. J. Schroeder, and N. M. Rouphail. Rationale for Incorporating
Queue Discharge Flow into _Highway Capacity Manual_ Procedure for
Analysis of Freeway Facilities. In _Transportation Research Record:_
_Journal of the Transportation Research Board, No. 2286_,
Transportation Research Board of the National Academies, Washington,
D.C., 2012, pp. 76–83.

8. Washburn, S. S., and D. S. Kirschner. Rural Freeway Level of Service
Based on Traveler Perception. In _Transportation Research Record:_
_Journal of the Transportation Research Board, No. 1988_,
Transportation Research Board of the National Academies, Washington,
D.C., 2006, pp. 31‒37.

9. Federal Highway Administration. _Highway Functional Classification_
_Concepts, Criteria and Procedures._ U.S. Department of Transportation,
Washington, D.C., 2013.

10. May, A. D., Jr., et al. _Capacity and Level of Service Analysis for_
_Freeway Facilities._ Fourth Interim Report. SAIC Corporation, McLean,
Va., March 1999.

11. Hajbabaie, A., N. Rouphail, B. Schroeder, and R. Dowling. A PlanningLevel Methodology for Freeway Facilities. In _Transportation Research_
_Record: Journal of the Transportation Research Board, No. 2483_,
Transportation Research Board, Washington, D.C., 2015, pp. 47–56.


