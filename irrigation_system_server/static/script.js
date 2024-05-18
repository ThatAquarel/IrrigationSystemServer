document.addEventListener('DOMContentLoaded', function () {
    new Sortable(document.getElementById('card-container'), {
        animation: 150,
        ghostClass: 'ghost',
        handle: '.card-handle',
    });
});

const times = [...Array(13).keys()].map(x => x*5);

const zones = ["f", "bp", "bs"]

for (const zone of zones) {
    let label = document.getElementById("time-" + zone)
    let slider = document.getElementById("slider-" + zone);

    slider.oninput = function() {
        label.innerHTML = times[slider.value] + " min";
    }

    slider.oninput();
}

