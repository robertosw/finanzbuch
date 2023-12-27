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

/// Will load the html to show a button in the navbar for each DepotEntry
async function navBarGetDepotEntryListHtml() {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

/// EventHandler for the button that shows a form to add one DepotEntry
async function navBarLoadHtmlAddDepotEntry() {
	var html = await invoke("get_html_depot_entry_add_form");
	document.getElementById("content").innerHTML = html;
}

/// EventHandler for the submit button of the form where a user can add an DepotEntry
async function addDepotEntryFormSubmit(event) {
	event.preventDefault();
	var name = document.getElementById('depotEntryAdd-Name').value;
	var variant = document.getElementById('depotEntryAdd-Selection').value;
	var sucessful = await invoke("depot_entry_add", { name: name, variant: variant });

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
	let sucessful = await invoke("depot_entry_delete", { depotEntryHash: hash });
	// TODO ^ use return value
	location.reload();	 // reload the page, so the deletion is rendered to UI
}

function depotEntryTableGetHtml() { depotEntryTableReloadHtml(this.event.target.dataset.hash); }

async function depotEntryTableReloadHtml(hash) {
	var html = await invoke("depot_entry_get_table_html", { depotEntryHash: hash });
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
	invoke("depot_entry_set_cell_value", {
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
	var sucessful = await invoke("depot_entry_add_previous_year", { depotEntryHash: hash });
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

async function depotOverviewRemoveComparison() {
	let html = await invoke("depot_overview_get_html_new_comparison", { action: "Remove" });
	document.getElementById("comparisonSelectionContainer").innerHTML = html;
}

async function depotOverviewAddComparison() {
	let html = await invoke("depot_overview_get_html_new_comparison", { action: "Add" });
	document.getElementById("comparisonSelectionContainer").innerHTML = html;
}

async function depotOverviewInitGraphs() {

	// replace page content
	let html = await invoke("depot_overview_get_html");
	document.getElementById("content").innerHTML = html;

	// Chart showing monthly value change of entire depot since start
	const fullDepotChartContext = document.getElementById('fullDepotChartContext');

	let fullDepotLabels = await invoke("depot_overview_alltime_get_labels");
	let datasets_json = await invoke("depot_overview_alltime_get_datasets");
	console.log(datasets_json);
	let datasets = JSON.parse(datasets_json);
	console.log(datasets);

	// TODO fill datasets entirely in rust
	// either as a normal key value map or by creating a struct in rust to hold that info

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
