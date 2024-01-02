// I couldn't care enough about getting ES6 Modules to work, just so this can be split into multiple files nicely
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
