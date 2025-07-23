function load_header() {
    setup_form(
        "authenticate",
        (data) => {
            return {
                "username": data.get("username"),
                "password": data.get("password"),
                "duration": data.get("remember_me") == "on" ? 43200 : 60,
            };
        },
        (response) => {
            if (response.status == 200) {
                response.json().then(body => {
                    if (document.getElementById("remember_me").checked) {
                        const expiry = new Date(Date.now() + 2592000000);
                        document.cookie = `api_key=${body.token}; expires=${expiry.toUTCString()}`;
                    } else {
                        document.cookie = `api_key=${body.token}`;
                    }
                    window.location.reload();
                });
            } else {
                response.json().then(body => {
                    alert(body.error)
                });
            }
        },
    );

    setup_form(
        "register",
        (data) => {
            return {
                "username": data.get("register_username"),
                "password": data.get("register_password"),
            };
        },
        (response) => {
            if (response.status == 204) {
                window.location.reload();
            } else {
                response.json().then(body => {
                    alert(body.error)
                });
            }
        },
    );
};

function logout() {
    document.cookie = "api_key=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    window.location.href = "/";
}

function redirect_to_home() {
    window.location.href = "/";
}

function show_auth_form() {
    const form1 = document.getElementById("auth-form");
    const form2 = document.getElementById("logout-form");

    if (form1) {
        form1.style.visibility = "visible";
    }

    if (form2) {
        form2.style.visibility = "visible";
    }

    document.getElementById("auth-background").style.visibility = "visible";
}

function hide_auth_form() {
    const form1 = document.getElementById("auth-form");
    const form2 = document.getElementById("logout-form");

    if (form1) {
        form1.style.visibility = "hidden";
    }

    if (form2) {
        form2.style.visibility = "hidden";
    }

    document.getElementById("auth-background").style.visibility = "hidden";
}

function switch_to_register() {
    document.getElementById("register").style.display = "flex";
    document.getElementById("authenticate").style.display = "none";
}

function switch_to_authenticate() {
    document.getElementById("register").style.display = "none";
    document.getElementById("authenticate").style.display = "flex";
}
