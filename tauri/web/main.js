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
	type: 'bar',
	data: {
		labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
		datasets: [{
			label: '# of Votes',
			data: [12, 19, 3, 5, 2, 3],
			borderWidth: 1
		}]
	},
	options: {
		scales: {
			y: {
				beginAtZero: true
			}
		}
	}
});
