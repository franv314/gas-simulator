window.onload = () => {
    load_header();
    document.getElementById("team_name").value = "";

    setup_form(
        "contest-editor",
        (data) => {
            return {
                "start_time": data.get("start_time"),
                "duration": parseInt(data.get("duration")),
                "drift": parseInt(data.get("drift")),
                "drift_time": parseInt(data.get("drift_time")),
                "jolly_time": parseInt(data.get("jolly_time")),
                "question_bonus": Array(10).fill(0).map((_, i) => parseInt(data.get(`question_bonus_${i + 1}`))),
                "contest_bonus": Array(10).fill(0).map((_, i) => parseInt(data.get(`contest_bonus_${i + 1}`))),
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

async function delete_team(contest_id, id) {
    if (!confirm("Stai per cancellare questa squadra! Sei sicuro?")) {
        return;
    }

    fetch(`/api/contests/${contest_id}/teams/${id}`, {
        method: "DELETE",
    }).then(response => {
        if (response.status == 204) {
            window.location.reload();
        } else {
            response.json().then(body => {
                alert(body.error)
            });
        }
    })
}

function reload_callback(response) {
    console.log(response);
    if (response.status == 201) {
        window.location.reload();
    } else {
        response.json().then(body => {
            alert(body.error)
        });
    }
}

function conv_add_team(data) {
    return {
        "team_name": data.get("team_name"),
    };
}
