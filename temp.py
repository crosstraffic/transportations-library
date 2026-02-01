import pymupdf.layout
import pymupdf4llm

# for i in range(1, 38):
#     doc = pymupdf.open(f"../resource/chap{i}.pdf")
#     md_text = pymupdf4llm.to_markdown(doc)
#     with open(f"chap{i}.md", "w", encoding="utf-8") as f:
#         f.write(md_text)
doc = pymupdf.open(f"../resource/Highway_Capacity_Manual_Edition_7.1_Chapters.pdf")
md_text = pymupdf4llm.to_markdown(doc)
with open("additional.md", "w", encoding="utf-8") as f:
    f.write(md_text)