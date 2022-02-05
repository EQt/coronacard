"""Remove unnecessary svg translate transformations."""
import re

from lxml import etree

xml = etree.parse("template.svg")
ns = {"s": xml.getroot().nsmap[None]}
for text in xml.xpath("//s:text", namespaces=ns):
    if (tra := text.attrib.get("transform")) is not None:
        if (m := re.match(r"translate\((.+) (.+)\)", tra)) is not None:
            x, y = map(float, m.groups())
            del text.attrib["transform"]
            for node in [text, *text.getchildren()]:
                if "x" in node.attrib:
                    node.attrib["x"] = f"{float(node.attrib['x']) + x:.5}"
                if "y" in node.attrib:
                    node.attrib["y"] = f"{float(node.attrib['y']) + y:.5}"
etree.dump(xml.getroot())
