import dominate
from dominate.tags import *
from dominate.util import raw
import xml.etree.ElementTree as ET
import datetime

tree = ET.parse("tests/reports/report.xml")
root = tree.getroot()

doc = dominate.document("Test Report")

overview = {}
results = {
    'lexer': {},
    'parser': {},
    'sem': {},
    'codegen': {},
    'regalloc': {}
}
cases = 0
passed = 0
elapsedTimeMs = 0

for child in root.findall("meta"):
    if 'time' in child.attrib:
        elapsedTimeMs = int(child.attrib['time'])

for child in root.findall("overview"):
    cases += int(child.attrib['total'])
    passed += int(child.attrib['passed'])
    overview[child.attrib['component']] = {
        "passed": child.attrib['passed'],
        "total": child.attrib['total']
    }

for child in root.findall("test"):
    comp = child.attrib['component']
    name = child.attrib['name']
    actual = child.attrib['actual']
    expected = child.attrib['expected']
    expOutput = 'N/A'
    actOutput = 'N/A'
    if 'expectedOutput' in child.attrib:
        expOutput = child.attrib['expectedOutput']
    if 'actualOutput' in child.attrib:
        actOutput = child.attrib['actualOutput']
    results[comp][name] = [actual, expected, actOutput, expOutput]

with doc.head:
    link(rel="stylesheet", href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css", crossorigin="anonymous")
    with style(type="text/css"):
        raw('body { margin: 0 auto; width: 70%; padding-bottom: 30px; } td.hd { font-weight: bold; text-align: center; } td { padding: 5px; }')

def tabhead():
    tab = table(cellspacing="1", cellpadding="5", border="1")
    with tab:
        with tr():
            td("Test name", cls="hd")
            td("Component", cls="hd")
            td("Exit code", cls="hd")
            td("Expected exit code", cls="hd")
            td("Program output", cls="hd")
            td("Expected output", cls="hd")
    return tab

def rahead():
    tab = table(cellspacing="1", cellpadding="5", border="1")
    with tab:
        with tr():
            td("Test name", cls="hd")
            td("Component", cls="hd")
            td("Correct", cls="hd")
            td("Accesses", cls="hd")
            td("Best", cls="hd")
    return tab

with doc:

    with div(cls="container", style="width:100%"):
        h2("Overview")
        with table(cellspacing="1", cellpadding=5, border=1):
            with tbody():
                with tr():
                    td("Date")
                    td(datetime.datetime.now().strftime("%d %b %Y %H:%M:%S"))
                with tr():
                    td("Generated in")
                    rawSeconds = elapsedTimeMs / 1000.0
                    minutes = int(rawSeconds) // 60
                    seconds = round(rawSeconds % 60, 2)
                    td(f"{str(minutes)}m {str(seconds)}s", style="text-align: left")
                with tr():
                    td("Test results")
                    with td():
                        with table(cellspacing="1", cellpadding=5, border=1):
                            with tbody():
                                with tr():
                                    td("Component", cls="hd")
                                    td("Results", cls="hd")
                                with tr():
                                    td(a("Part I: Lexer", href="#lexer"))
                                    td(f"{overview['lexer']['passed']} / {overview['lexer']['total']}")
                                with tr():
                                    td(a("Part I & II: Parser", href="#parser"))
                                    td(f"{overview['parser']['passed']} / {overview['parser']['total']}")
                                with tr():
                                    td(a("Part II: Semantics", href="#sem"))
                                    td(f"{overview['sem']['passed']} / {overview['sem']['total']}")
                                with tr():
                                    td(a("Part III: Codegen", href="#codegen"))
                                    td(f"{overview['codegen']['passed']} / {overview['codegen']['total']}")
                                with tr():
                                    td(a("Part IV: Register Allocation", href="#regalloc"))
                                    td(f"{overview['regalloc']['passed']} / {overview['regalloc']['total']}")
                                with tr():
                                    td("Total", style="text-align: left")
                                    td(f"{passed} / {cases}", style="text-align: left")
    with div(cls="container", style="width:100%"):
        h1("Detailed results")
        for component in results.keys():
            if component == 'lexer':
                div(h1("Part I: Lexer"), id=component)
            elif component == 'parser':
                div(h1("Part I & II: Parser"), id=component)
            elif component == 'sem':
                div(h1("Part II: Semantic Analysis"), id=component)
            elif component == 'codegen':
                div(h1("Part III: Code Generation"), id=component)
            elif component == 'regalloc':
                div(h1("Part IV: Register Allocation"), id=component)
            with tabhead():
                with tbody():
                    for test in sorted(results[component].keys()):
                        actual = results[component][test][0]
                        expected = results[component][test][1]
                        actOutput = results[component][test][2]
                        expOutput = results[component][test][3]
                        if actual == expected:
                            td_class = 'alert-success'
                        else:
                            td_class = 'alert-danger'
                        with tr():
                            td(test, cls=td_class)
                            td(component, cls=td_class)
                            td(actual, cls=td_class)
                            td(expected, cls=td_class)
                            td(actOutput, cls=td_class, style="white-space: pre")
                            td(expOutput, cls=td_class, style="white-space: pre")

print(doc)