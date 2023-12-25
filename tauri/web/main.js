// I couldn't care enough about getting ES6 Modules to work, so this can be split into multiple files
const { invoke } = window.__TAURI__.tauri;

// Naming convention I try to follow + example
// 	 <Part of the UI><what does it do><for what>
//   depotEntryTable GetHtml                       
//   navBar          LoadHtml         AddDepotEntry

/// Only works in async functions, simply waits some time
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

// -------------------- Init / Navbar -------------------- //
window.onload = () => { navBarGetDepotEntryListHtml(); }

window.onresize = () => { resizeDepotOverviewGraphs(); };

function resizeDepotOverviewGraphs() {
	// TODO this wont work properly

	// 1. display: none on all canvas
	let graphList = document.querySelectorAll("canvas.chartjs");

	for (let graph of graphList) {
		graph.style.display = "none";
		graph.style.display = ""; // to show again
	}

	// 2. Let parents of the canvas resize using their css rules
	// 3. Copy their height & width

	// 4. show canvas again
	// 5. force canvas to use the height & width of their parent from step 3

	// Current state is that resizing of the graphs is dependent on the width alone.
	// Is doesnt care about the height changing

	// let allChartsContainer = document.querySelector("div#depotOverviewAllChartsContainer");
	// let allChartsContainerClientHeight = allChartsContainer.clientHeight;	// This is just a number, so add "px" to it when assigning

	// let depotOverviewChartContainerList = document.querySelectorAll("div.depotOverviewChartContainer");

	// for (let chartContainer of depotOverviewChartContainerList) {
	// 	chartContainer.style.maxHeight = allChartsContainerClientHeight + "px";
	// 	chartContainer.style.height = allChartsContainerClientHeight + "px";
	// }
	// console.log(depotOverviewChartContainerList);
}

/// Will load the html to show a button in the navbar for each DepotEntry
async function navBarGetDepotEntryListHtml() {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

/// EventHandler for the button that shows a form to add one DepotEntry
async function navBarLoadHtmlAddDepotEntry() {
	var html = await invoke("get_html_add_depot_entry_form");
	document.getElementById("content").innerHTML = html;
}

/// EventHandler for the submit button of the form where a user can add an DepotEntry
async function addDepotEntryFormSubmit(event) {
	event.preventDefault();
	var name = document.getElementById('depotEntryAdd-Name').value;
	var variant = document.getElementById('depotEntryAdd-Selection').value;
	var sucessful = await invoke("add_depot_entry", { name: name, variant: variant });

	if (sucessful) {
		navBarGetDepotEntryListHtml();
		document.getElementById('depotEntryAdd-Name').value = "";
	} else {
		console.warn("addDepotEntryFormSubmit failed");
		var buttonElement = document.getElementById("depotEntryAddFormDoneBtn");
		var innerTextBefore = buttonElement.innerHTML;
		buttonElement.innerHTML = "Error adding this entry";
		buttonElement.classList.add('error');
		await sleep(3000);
		buttonElement.innerHTML = innerTextBefore;	// Reset text
		buttonElement.classList.remove('error');
		return;
	}
}

// -------------------- DepotEntries -------------------- //

async function depotEntryTableDeleteEntry() {
	let hash = this.event.target.dataset.hash;
	let sucessful = await invoke("delete_depot_entry", { depotEntryHash: hash });
	// TODO ^ use return value
	location.reload();	 // reload the page, so the deletion is rendered to UI
}

function depotEntryTableGetHtml() { depotEntryTableReloadHtml(this.event.target.dataset.hash); }

async function depotEntryTableReloadHtml(hash) {
	var html = await invoke("get_depot_entry_table_html", { depotEntryHash: hash });
	document.getElementById("content").innerHTML = html;

	// scroll to this years table (bottom of page)
	// without the timeout, this would ignore the padding of content and not scroll far enough ..
	setTimeout(() => { window.scrollBy(0, document.getElementById("content").scrollHeight); }, 50);
}

async function depotEntryTableSetCell() {
	var [field_type, year, month, hash] = this.event.target.id.split('-');
	var field = "";

	switch (field_type) {
		case "itp":
			field = "PricePerUnit";
			break;

		case "its":
			field = "Amount";
			break;

		case "ita":
			field = "AdditionalTransactions";
			break;
	}

	// TODO check for return value
	invoke("set_depot_entry_table_cell", {
		depotEntryHash: hash,
		field: field,
		value: this.event.target.textContent,
		year: parseInt(year),
		month: parseInt(month)
	});
}

/// add new year + reload table html
async function depotEntryTableAddYear() {
	var buttonElement = this.event.target;

	var hash = buttonElement.dataset.hash;
	var sucessful = await invoke("add_depot_entrys_previous_year", { depotEntryHash: hash });
	console.log("depotEntryTableAddYear " + sucessful);

	if (!sucessful) {
		console.warn("Previous Year could not be added to this depotEntry: " + hash);
		var innerTextBefore = buttonElement.innerHTML;
		buttonElement.innerHTML = "An Error occurred";
		buttonElement.classList.add('error');
		await sleep(3000);
		buttonElement.innerHTML = innerTextBefore;	// Reset text
		buttonElement.classList.remove('error');
		return;
	}

	depotEntryTableReloadHtml(hash);
}

function depotEntryTableScrollToRow(rowId) {
	let elem = document.getElementById(rowId);
	elem.scrollIntoView({
		behavior: 'smooth',
		block: 'center',
	});
}

// -------------------- DepotOverview -------------------- //

async function depotOverviewInitGraphs() {

	// replace page content
	let html = await invoke("depot_overview_get_html");
	document.getElementById("content").innerHTML = html;

	// Chart showing monthly value change of entire depot since start
	const fullDepotChartContext = document.getElementById('fullDepotChartContext');

	let fullDepotLabels = await invoke("depot_overview_alltime_get_labels");
	let fullDepotData = await invoke("depot_overview_alltime_get_data");
	let prognosis_7 = await invoke("depot_overview_alltime_get_prognosis", { growthRate: 0.07 });
	let prognosis_5 = await invoke("depot_overview_alltime_get_prognosis", { growthRate: 0.05 });

	new Chart(fullDepotChartContext, {
		data: {
			labels: fullDepotLabels,
			datasets: [
				{
					type: 'line',
					label: 'Depot value',
					data: fullDepotData,
					borderColor: 'rgb(0, 0, 0)',
					order: 1,
					fill: true,
					cubicInterpolationMode: 'monotone',	// better than tension, because the smoothed line never exceeed the actual value
					spanGaps: false,		// x values without a y value will produce gaps in the line
				},
				{
					type: 'line',
					label: 'Prognosis 5%',
					data: prognosis_5,
					borderColor: 'rgba(0, 200, 0, 1)',
					order: 2,
					borderDash: [1, 8],
					borderCapStyle: 'round',
				},
				{
					type: 'line',
					label: 'Prognosis 7%',
					data: prognosis_7,
					borderColor: 'rgba(0, 0, 200, 1)',
					order: 3,
					borderDash: [1, 8],
					borderCapStyle: 'round',
				}
			]

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
