import React from 'react';
import QrScanner from './QrScanner';

function App() {
    const handleScan = (data) => {
        console.log('QR Code Data:', data);
        // Process the scanned data
    };

    return (
        <div className="App">
            <header className="App-header">
                <QrScanner onScan={handleScan} />
            </header>
        </div>
    );
}

export default App;

