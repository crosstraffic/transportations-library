import pymupdf4llm

md_text = pymupdf4llm.to_markdown("../resource/chap2.pdf")
with open("chap2.md", "w", encoding="utf-8") as f:
    f.write(md_text)