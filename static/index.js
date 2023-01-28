// Get the found devices and put them in the dropdown
let dropdown = document.querySelector("#deviceList");
let xmlHttp = new XMLHttpRequest();
xmlHttp.open("GET", "/devices", false);
xmlHttp.send(null);
xmlHttp.responseText.split("\r\n")[0].split(",").forEach((device) => {
	let newOption = document.createElement("option");
	newOption.setAttribute("value", device);
	newOption.appendChild(document.createTextNode(device));
	dropdown.appendChild(newOption);
});

// Register an event listener on every keypress button
// to make it control the TV
document.querySelectorAll(".keypress").forEach((button) => {
	const command = button.id;
	button.addEventListener("click", () => {
		let commandReq = new XMLHttpRequest();
		commandReq.open("PUT", "/keypress", false);
		commandReq.send("device=" + document.querySelector("#deviceList").value + "&action=" + command);
	});
});

// Menu control
document.querySelector("#Features").addEventListener("click", () => {
	let menu = document.querySelector("#menu");
	if (menu.style.display == "none" || menu.style.display == "")
	{
		menu.style.display = "flex";
	}
	else
	{
		menu.style.display = "none";
	}
});

document.querySelector("#text").addEventListener("input", (event) => {
	let commandReq = new XMLHttpRequest();
	const device = document.querySelector("#deviceList").value;

	// If the user typed a letter, number, or other symbol
	// if (event.key.length == 1)
	// {
	// 	commandReq.open("PUT", "/keypress", false);
	// 	commandReq.send("device=" + device + "&action=Lit_" + event.key);
	// }
	// else if (event.key == "Backspace" || event.key == "Enter")
	// {
	// 	commandReq.open("PUT", "/keypress", false);
	// 	commandReq.send("device=" + device + "&action=" + event.key);
	// }
});