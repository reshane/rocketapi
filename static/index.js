console.log("Hello, World!");
const url = "/csgo_round_snapshots.csv";
const data = {
    "query": "shane was here"
};

function submitButtonClick() {
    const input = document.getElementById("query-text").value;
    document.getElementById("query-text").value = "";
    console.log(input);
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
