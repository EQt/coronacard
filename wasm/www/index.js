import init, {gen_card, version} from "./pkg/coronacard_wasm.js";

await init();

export const wasm_version = () => "v" + version();

// adapted from https://stackoverflow.com/a/45831280
function download(blob, filename) {
    let element = document.createElement('a');
    let url = URL.createObjectURL(blob)
    element.setAttribute('href', url);
    element.setAttribute('download', filename);
    element.style.display = 'none';
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
}

export function convert() {
    console.log("Convert button pressed!");
    const convertButtonElement = document.getElementById("convertButton");
    const convertLabel = convertButtonElement.innerHTML;
    const selectedFile = document.getElementById("formFile").files[0];
    console.log(`Selected file: ${selectedFile}`);
    if (selectedFile === undefined) {
        alert("Please select your file first!");
        return;
    } else {
        convertButtonElement.innerHTML = "Computing...";
        const fileData = new Blob([selectedFile]);
        new Promise(function (resolve) {
            let reader = new FileReader();
            reader.readAsArrayBuffer(fileData);
            reader.onload = function (e) {
                resolve(new Uint8Array(e.target.result));
            }
        }).then(function (bytesArr) {
            const is_pdf = document.getElementsByName("format")[0].checked;
            const din_a4 = document.getElementsByName("size")[0].checked;
            console.log(`run wasm (${selectedFile}, is_pdf=${is_pdf}, din_a4=${din_a4})`);
            const f = gen_card(bytesArr, din_a4, is_pdf);
            const blob = new Blob([f.content()], { type: f.mimetype() });
            const output_filename = is_pdf ? "corona_card.pdf" : "corona_card.svg";
            console.log("Showing SaveAs dialog to the user...");
            download(blob, output_filename);
            convertButtonElement.innerHTML = convertLabel;
        }).catch(function (err) {
            console.log(err);
            alert(`Error: ${err}`)
            convertButtonElement.innerHTML = convertLabel;
        });
        console.log("Done!")
    }
}


// sets the input file field to selected file name
export function updateInputField() {
    let inputElem = document.getElementById("imageInputFile");
    let selectedFile = inputElem.files[0]
    let fileNameField = document.getElementById("fileNameField")
    fileNameField.innerHTML = selectedFile.name;
}
