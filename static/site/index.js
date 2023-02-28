
const url = "csgodata";
const data = {
    "query": "shane was here"
};

function submitButtonClick() {
    const phrase1 = document.querySelector("input[name='phrase1']").value;
    const phrase2 = document.querySelector("input[name='phrase2']").value;
    console.log(phrase1);
    console.log(phrase2);
    let data = {
        "phrase1": phrase1,
        "phrase2": phrase2
    };
    let num = fetch("/shane", {
        method: 'POST',
        body: JSON.stringify(data)
    }).then((response) => response.json())
    .then((data) => document.querySelector(".dist").innerText = data);
}

function submitButtonClick1() {
    const input = document.getElementById("phrase1").value;
    document.getElementById("phrase1").value = "";
    if (input === "shane is cool") {
        fetch(url, {
            method: 'GET'
        })
        .then( res => res.blob() )
        .then( blob => {
            var file = window.URL.createObjectURL(blob);
            window.location.assign(file);
        });
    }
}
