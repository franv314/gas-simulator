window.onload = () => {
    load_header();
    setup_submitter();

    document.addEventListener('fullscreenchange', exit_fullscreen_adjust, false);

    setup_flipdown();
};

setInterval(reload_content, 15000); // reload ranking every 15 seconds

function setup_flipdown() {
    document.getElementById("flipdown").innerHTML = "";
    document.getElementById("clock-text").innerHTML = "";

    const url = window.location.href.split("/");
    const id = url[url.length - 1];

    fetch(`/api/contests/${id}`).then(res => res.json()).then(res => {
        const start_date = new Date(`${res.start_time}Z`).getTime();

        if (start_date > new Date().getTime()) {
            var flipdown = new FlipDown(start_date / 1000).start().ifEnded(setup_flipdown);
            document.getElementById("clock-text").innerHTML = "La gara non è ancora iniziata";
        } else if (start_date + 1000 * res.duration < new Date().getTime()) {
            document.getElementById("clock-text").innerHTML = "La gara è terminata";
            document.getElementById("flipdown").style.display = "none";
        } else {
            var flipdown = new FlipDown(start_date / 1000 + res.duration).start().ifEnded(setup_flipdown);
        }
    });
}

function reload_content() {
    const hidden_teams = document.getElementById("toggle-visibility").getAttribute("onclick") == "show_fake_teams()";

    fetch(window.location.href).then(body => body.text()).then(text => {
        const parser = new DOMParser();
        const doc = parser.parseFromString(text, "text/html");

        const ranking = doc.getElementById("ranking");

        document.getElementById("ranking").innerHTML = ranking.innerHTML;

        if (hidden_teams) {
            hide_fake_teams();
        }
    })
}

function hide_fake_teams() {
    Array.from(document.getElementsByClassName("fake-team")).forEach(elem => {
        elem.setAttribute("hidden", "");
    });

    document.getElementById("toggle-visibility").setAttribute("onclick", "show_fake_teams()");
    document.getElementById("toggle-visibility-text").innerText = "Mostra squadre fantasma";
    document.getElementById("show-teams-icon").style.display = "flex";
    document.getElementById("hide-teams-icon").style.display = "none";
}

function show_fake_teams() {
    Array.from(document.getElementsByClassName("fake-team")).forEach(elem => {
        elem.removeAttribute("hidden");
    });

    document.getElementById("toggle-visibility").setAttribute("onclick", "hide_fake_teams()");
    document.getElementById("toggle-visibility-text").innerText = "Nascondi squadre fantasma";
    document.getElementById("show-teams-icon").style.display = "none";
    document.getElementById("hide-teams-icon").style.display = "flex";
}

function show_submitter() {
    document.getElementById("submitter-background").style.visibility = "visible";
    document.getElementById("submitter-wrapper").style.visibility = "visible";
    document.body.style.overflow = "hidden";
}

function hide_submitter() {
    document.getElementById("submitter-background").style.visibility = "hidden";
    document.getElementById("submitter-wrapper").style.visibility = "hidden";
    document.body.style.overflow = "auto";
}

function enter_fullscreen() {
    document.getElementById("enter-fullscreen-btn").style.display = "none";
    document.getElementById("exit-fullscreen-btn").style.display = "flex";
    document.querySelector("header").style.display = "none";
    document.querySelector("footer").style.display = "none";
    document.getElementById("buttons").style.display = "none";
    document.getElementById("fullscreen-buttons").style.top = "10px";

    if (document.body.requestFullscreen)
        document.body.requestFullscreen();
}

function exit_fullscreen_adjust() {
    if (!document.webkitIsFullScreen && !document.mozFullScreen && !document.msFullscreenElement) {
        document.getElementById("enter-fullscreen-btn").style.display = "flex";
        document.getElementById("exit-fullscreen-btn").style.display = "none";
        document.querySelector("header").style.display = "flex";
        document.querySelector("footer").style.display = "flex";
        document.getElementById("buttons").style.display = "flex";
        document.getElementById("fullscreen-buttons").style.top = "60px";
    }
}

function exit_fullscreen() {
    if (document.exitFullscreen)
        document.exitFullscreen();
}
