function pad(number) {
    return number <= 9 ? `0${number}` : `${number}`;
}

window.onload = () => {
    load_header();

    setup_form(
        "contest-editor",
        (data) => {
            return {
                "phiquadro_id": parseInt(data.get("phiquadro_id")),
                "phiquadro_sess": parseInt(data.get("phiquadro_sess")),
                "name": data.get("name"),
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
            if (response.status == 201) {
                response.json().then((body) => {
                    history.pushState({}, "");
                    window.location.replace(`settings/${body.contest_id}`);
                });
            } else {
                response.json().then(body => {
                    alert(body.error)
                });
            }
        },
    );

    const now = new Date();
    const datetime = new Date(now.getTime() + 300000); // five minutes from now
    document.getElementById("start_time").setAttribute("value", `${datetime.getFullYear()}-${pad(datetime.getMonth() + 1)}-${pad(datetime.getDate())} ${pad(datetime.getHours())}:${pad(datetime.getMinutes())}`);
};
