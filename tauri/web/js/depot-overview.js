/// Only works in async functions, simply waits some time
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

async function depotOverviewRemoveComparison() {
    await invoke("depot_overview_do_comparison_action", { action: "Remove" });
    depotOverviewInitialize();
}

async function depotOverviewAddComparison() {
    await invoke("depot_overview_do_comparison_action", { action: "Add" });
    depotOverviewInitialize();
}

async function depotOverviewInitialize() {

    // replace page content
    let html = await invoke("depot_overview_get_html");
    document.getElementById("content").innerHTML = html;

    // Chart showing monthly value change of entire depot since start
    const fullDepotChartContext = document.getElementById('fullDepotChartContext');

    let fullDepotLabels = await invoke("depot_overview_alltime_get_labels");
    let datasets = await invoke("depot_overview_alltime_get_datasets");

    let datasetConfigAll = {
        type: "line",
        spanGaps: false,
        borderCapStyle: "round",
        cubicInterpolationMode: "monotone",
        pointStyle: "triangle",
        pointRadius: 4,
        pointHoverRadius: 10,
    };

    datasets.forEach(function (el, index, array) { array[index] = { ...array[index], ...datasetConfigAll }; });

    let depotDataConfig = {
        borderColor: "hsla(220, 100%, 60%, 1)",
        backgroundColor: "hsla(220, 100%, 60%, 0.1)",
        fill: "start",
        order: 2,
    };
    let transactionDataConfig = {
        borderColor: "hsl(280, 50%, 65%)",
        backgroundColor: "hsla(280, 50%, 65%, 0.3)",
        fill: "start",
        order: 1,
        hidden: true,
    };
    let prognosisDataConfig = {
        borderColor: "hsla(30, 0%, 40%, 0.5)",
        backgroundColor: "hsla(30, 0%, 40%, 0.25)",	// they arent filled, but bg color is used in legend two
        borderDash: [1, 5],
        fill: false,
        pointStyle: false,
        order: 3,
    };

    // join datasets and their additional config
    datasets[0] = { ...datasets[0], ...depotDataConfig };
    datasets[1] = { ...datasets[1], ...transactionDataConfig };
    datasets.forEach(function (el, index, array) {
        // for all prognosis configs
        if (index >= 2) { array[index] = { ...array[index], ...prognosisDataConfig }; }
    });

    console.log(datasets);

    new Chart(fullDepotChartContext, {
        data: {
            labels: fullDepotLabels,
            datasets: datasets,
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                y: {
                    beginAtZero: false
                }
            }
        }
    });
}

async function depotOverviewOnInputComparison() {
    await invoke("depot_overview_change_comparison", {
        comparisonId: this.event.target.dataset.id,
        newValue: this.event.target.value
    });
}
