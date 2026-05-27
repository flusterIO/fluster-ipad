from alphabetic import WritingSystem
from pathlib import Path
import json

ws = WritingSystem()

data = []


def generateAlphabets():
    for lang in ws.Language:
        data.append(
            {"Language": lang.name, "Alphabet": ws.by_language(lang)[lang.name]}
        )


if __name__ == "__main__":
    generateAlphabets()
    p = (
        Path(__file__).parent.parent.parent
        / "packages/rust/fluster_core_utilities/src/code_gen/internationalization/alphabets.json"
    )
    json_content = json.dumps(data)

    p.write_text(json_content)
