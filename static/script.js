document.addEventListener('DOMContentLoaded', function () {
    new Sortable(document.getElementById('card-container'), {
        animation: 150,
        ghostClass: 'ghost',
        handle: '.card-handle',
    });
});

const times = [...Array(13).keys()].map(x => x*5);
const zones = ["f", "bp", "bs"]

function total_time() {
    let sum = zones.reduce((previous, current) => {
        let label = document.getElementById("time-" + current);
        return previous + label.value;
    }, 0);

    let start_label = document.getElementById("start-label");
    start_label.innerHTML = "Total " + sum + " min";
}

for (const zone of zones) {
    let label = document.getElementById("time-" + zone);
    let slider = document.getElementById("slider-" + zone);

    slider.oninput = function() {
        let time = times[slider.value];
        label.innerHTML = time + " min";
        label.value = time;

        total_time();
    }

    slider.oninput();
}

function requestStatus() {
    updateStatus((response) => {
        alert(JSON.stringify(response, null, 4));
    });
}

function updateStatus(callback) {
    fetch('/routine/status')
    .then(response => {
        if (!response.ok) {
            throw new Error('Unable to fetch status');
        }
        return response;
    })
    .then(response => {
        return response.json();
    })
    .then(response => {
        callback(response);

        if (response.running) {
            return "Current routine";
        }
        return "No current routine";
    })
    .then(data => {
        let stop_label = document.getElementById("stop-label");
        stop_label.innerHTML = data;
    })
    .catch(error => {
        console.error('Unable to fetch status: ', error);
    });
}
updateStatus(_=>{});

let development_mode = false;

function startRoutine(event) {
    const form = event.target;
    let formData = new FormData(form);

    if (!development_mode) {
        for (const a of formData.keys()) {
            if (!a.includes("duration")) {
                continue;
            }
    
            let duration = 300 * Number(formData.get(a));
            formData.set(a, duration.toString());   
        }
    }

    const params = new URLSearchParams(formData).toString();

    fetch(`/routine/run?${params}`)
    .then(response => response.json())
    .then(data => {
        alert(JSON.stringify(data, null, 4));
    });
}

function stopRoutine(event) {
    fetch(`/routine/stop`)
    .then(response => response.json())
    .then(data => {
        alert(JSON.stringify(data, null, 4));
    });
}

function developmentMode(event) {
    const checkbox = event.target;
    development_mode = checkbox.checked;
}
