import React, { useEffect, useRef } from 'react';
import { Html5QrcodeScanner } from 'html5-qrcode';

const QrScanner = ({ onScan }) => {
    const qrRef = useRef(null);

    useEffect(() => {
        // Ensure the element is present
        if (qrRef.current) {
            const qrScanner = new Html5QrcodeScanner(
                qrRef.current.id, 
                { fps: 10, qrbox: 250 },
                false
            );

            qrScanner.render(onScan, onError);

            return () => qrScanner.clear();
        }
    }, [onScan]);

    const onError = (error) => {
        console.error(error);
    };

    return <div ref={qrRef} id="qr-scanner"></div>;
};

export default QrScanner;

