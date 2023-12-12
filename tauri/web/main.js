// I couldn't care enough about getting ES6 Modules to work, so this can be split into multiple files
const { invoke } = window.__TAURI__.tauri;


/// Only works in async functions, simply waits some time
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

// -------------------- Init / Navbar -------------------- //
window.onload = () => { navBarLoadDepotEntryList(); }

async function navBarLoadDepotEntryList() {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

async function navBarBtnAddDepotEntry() {
	var html = await invoke("get_html_add_depot_entry_form");
	document.getElementById("content").innerHTML = html;
}

async function addDepotEntryFormSubmit(event) {
	event.preventDefault();
	var name = document.getElementById('depotEntryAdd-Name').value;
	var variant = document.getElementById('depotEntryAdd-Selection').value;
	var sucessful = await invoke("add_depot_entry", { name: name, variant: variant });

	if (sucessful) {
		navBarLoadDepotEntryList();
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

async function getDepotEntryTableHtml() { replaceDepotEntryTableHtml(this.event.srcElement.name); }

async function replaceDepotEntryTableHtml(hash) {
	var html = await invoke("get_depot_entry_table_html", { depotEntryHash: hash });
	document.getElementById("content").innerHTML = html;

	// scroll to this years table (bottom of page)
	// without the timeout, this would ignore the padding of content and not scroll far enough ..
	setTimeout(() => { window.scrollBy(0, document.getElementById("content").scrollHeight); }, 50);
}

async function setDepotEntryTableCell() {
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
async function addDepotTable() {
	var buttonElement = this.event.target;

	var hash = buttonElement.name;
	var sucessful = await invoke("add_depot_entrys_previous_year", { depotEntryHash: hash });
	console.log("addDepotTable " + sucessful);

	if (!sucessful) {
		console.warn("Previous Year could not be added to this depotEntry: " + buttonElement.name);
		var innerTextBefore = buttonElement.innerHTML;
		buttonElement.innerHTML = "An Error occurred";
		buttonElement.classList.add('error');
		await sleep(3000);
		buttonElement.innerHTML = innerTextBefore;	// Reset text
		buttonElement.classList.remove('error');
		return;
	}

	replaceDepotEntryTableHtml(hash);
}

function scrollDepotTableToRow(rowId) {
	let elem = document.getElementById(rowId);
	elem.scrollIntoView({
		behavior: 'smooth',
		block: 'center',
	});
}

// -------------------- DepotOverview -------------------- //

const ctx = document.getElementById('myChart');

new Chart(ctx, {
	data: {
		labels: ['2023-01', '2023-02', '2023-03', '2023-04', '2023-05', '2023-06'],
		datasets: [
			{
				type: 'line',
				label: 'Depot value',
				data: [6, 8, 3, 5, 2, 3],
				borderColor: 'rgb(0, 0, 0)',
				order: 1,
				fill: true,
				cubicInterpolationMode: 'monotone',	// better than tension, because the smoothed line never exceeed the actual value
				spanGaps: false,		// x values without a y value will produce gaps in the line
			},
			{
				type: 'line',
				label: 'Prognosis 5%',
				data: [6, 6.3, 6.615, 6.94575, 7.2933, 7.665],
				borderColor: 'rgba(0, 200, 0, 1)',
				order: 2,
				borderDash: [1, 8],
				borderCapStyle: 'round',
			},
			{
				type: 'line',
				label: 'Prognosis 7%',
				data: [6, 6.42, 6.8694, 7.350258, 7.86477606, 8.415310384],
				borderColor: 'rgba(0, 0, 200, 1)',
				order: 3,
				borderDash: [1, 8],
				borderCapStyle: 'round',
			}
		]

	},
	options: {
		scales: {
			y: {
				beginAtZero: false
			}
		}
	}
});
