import init, {gen_card} from "./pkg/coronacard_wasm.js";

init();

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
    let convertButtonElement = document.getElementById('convertButton');
    let selectedFile = document.getElementById('imageInputFile').files[0];
    console.log(`Selected file: ${selectedFile}`);
    if (selectedFile === undefined) {
        alert("Please select your file first!");
        return;
    } else {
        convertButtonElement.innerHTML = 'Computing...';
        const fileData = new Blob([selectedFile]);

        new Promise(function (resolve) {
            let reader = new FileReader();
            reader.readAsArrayBuffer(fileData);
            reader.onload = function (e) {
                resolve(new Uint8Array(e.target.result));
            }
        }).then(function (bytesArr) {
            console.log(`run wasm (${selectedFile})`);
            const f = gen_card(bytesArr, true);
            const blob = new Blob([f.content()], { type: f.mimetype() });
            const output_filename = "corona_card.pdf";
            console.log("Showing SaveAs dialog to the user...");
            download(blob, output_filename);
            convertButtonElement.innerHTML = "Convert";
        }).catch(function (err) {
            console.log(err);
            alert(`Error: ${err}`)
            convertButtonElement.innerHTML = "Convert";
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
