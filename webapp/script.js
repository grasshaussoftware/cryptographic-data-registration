function generateQRCode() {
    var name = document.getElementById("name").value;
    var dob = document.getElementById("dob").value;
    var pob = document.getElementById("pob").value;
    var phone = document.getElementById("phone").value;
    var email = document.getElementById("email").value;  // Retrieving the email value

    // Generate a unique serial number
    var serialNumber = 'SN' + Date.now() + Math.floor(Math.random() * 1000);

    // Concatenate the data for QR code generation, including the serial number
    var qrData = `Name: ${name}, DOB: ${dob}, Citizen: ${pob}, Phone: ${phone}, Email: ${email}, Serial: ${serialNumber}`;

    // Display data for double-checking in a vertically stacked manner
    var infoPreview = document.getElementById("infoPreview");
    infoPreview.innerHTML = `<strong>Confirm Your Information:</strong><br>` +
                            `Name: ${name}<br>` +
                            `DOB: ${dob}<br>` +
                            `Citizen: ${pob}<br>` +
                            `Phone: ${phone}<br>` +
                            `Email: ${email}<br>` +
                            `Serial Number: ${serialNumber}`;

    // Get the QR code container, clear previous contents, and set background
    var qrContainer = document.getElementById("qrCode");
    qrContainer.innerHTML = "";
    qrContainer.style.visibility = 'visible'; // Make the container visible
    qrContainer.style.backgroundColor = 'purple'; // Setting the specified background color

    // Generate a new QR code
    new QRCode(qrContainer, {
        text: qrData,
        width: 256, // Increased QR code size for better readability
        height: 256,
        colorDark: "#000000",
        colorLight: "#ffffff" // Ensuring the QR code background is white
    });
}
