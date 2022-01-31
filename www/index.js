import init, {gen_svg} from "./pkg/coronacard_wasm.js";

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
    let inputFileExtension = ".png";
    let outFiletypeElement = document.getElementById('inlineFormCustomSelectPref');
    console.log(`Selected file: ${selectedFile}`);
    if (selectedFile === undefined) {
        alert("Please select your file first!");
        return;
    } else {
        convertButtonElement.innerHTML = 'Computing...';
        let fileData = new Blob([selectedFile]);

        // returns a byte array of file contents
        let promise = new Promise(function (resolve) {
            let reader = new FileReader();
            reader.readAsArrayBuffer(fileData);
            reader.onload = function (e) {
                let arrayBuffer = e.target.result;
                console.log(arrayBuffer);
                let bytes = new Uint8Array(arrayBuffer);
                resolve(bytes);
            }
        });
        promise.then(function (bytesArr) {
            console.log(`run wasm (${selectedFile})`);
            let retBytes = gen_svg(bytesArr, true);
            console.log("Done");
            let blob = new Blob([retBytes], { type: "text/plain" });
            let output_filename = "card.txt";
            console.log("Showing SaveAs dialog to the user...");
            download(blob, output_filename);
            convertButtonElement.innerHTML = "Convert";
        }).catch(function (err) {
            console.log(err);
            alert(":( Error occured, please reload and try again.");
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
