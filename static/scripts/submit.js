window.onload = () => {
    load_header();
    setup_submitter();
}

function toggle_spoiler(id) {
    document.getElementById(`answer-${id}`).classList.toggle("visible");
}

function reload_content() {
    fetch(window.location.href).then(body => body.text()).then(text => {
        const parser = new DOMParser();
        const doc = parser.parseFromString(text, "text/html");

        const submissions = doc.getElementById("submissions-container");
        const jollies = doc.getElementById("jollies-container");

        document.getElementById("submissions-container").innerHTML = submissions.innerHTML;
        document.getElementById("jollies-container").innerHTML = jollies.innerHTML;

        document.getElementById("submissions-container").style.display = submissions.style.display;
        document.getElementById("jollies-container").style.display = jollies.style.display;
    });
}

function show_submission_result(correct) {
    const textbox = document.getElementById("answer-input");
    textbox.classList.remove("true", "false");
    textbox.classList.add(correct);

    document.getElementById("submitter").reset();
    reload_content();
}
