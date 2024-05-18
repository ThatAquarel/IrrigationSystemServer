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

fetch('/routine/status')
    .then(response => {
        if (!response.ok) {
            throw new Error('Unable to fetch status');
        }
        return response.text();
    })
    .then(data => {
        let stop_label = document.getElementById("stop-label");
        stop_label.innerHTML = data;
    })
    .catch(error => {
        console.error('Unable to fetch status: ', error);
    });
