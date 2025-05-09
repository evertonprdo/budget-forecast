function updateSliderValue(sliderId) {
    const slider = document.getElementById(sliderId);
    const output = document.getElementById(sliderId + '-value');
    
    const formatValue = (id, value) => {
        switch(id) {
            case 'slider1':
            case 'slider2':
                return value.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' });
            case 'slider4':
                return `${(value * 100).toFixed(1)}%`;
            default:
                return value;
        }
    };
    
    output.textContent = formatValue(sliderId, parseFloat(slider.value));
    
    slider.oninput = async function() {
        output.textContent = formatValue(sliderId, parseFloat(this.value));
    }
}

function requestForecast(buttonId) {
    const button = document.getElementById(buttonId);

    button.onclick = async () => {
        const outflow = document.getElementById('slider1').value;
        const inflow = document.getElementById('slider2').value;
        const range = document.getElementById('slider3').value;
        const inflation = document.getElementById('slider4').value;
        const offset = document.getElementById('slider5').value;

        const url = "/api/forecast";
        try {
            const response = await fetch(url, {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    outflow: parseFloat(outflow),
                    inflow: parseFloat(inflow),
                    range: parseInt(range),
                    inflation_rate: parseFloat(inflation),
                    inflow_offset: parseInt(offset),
                })
            });
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const data = await response.json();
            updateResults(data);
        } catch (error) {
            console.error(error.message);
        }
    }
}

function updateResults(data) {
    const formatBRL = (value) => value.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' });
    
    // Update summary fields
    document.getElementById('outflow-year').textContent = formatBRL(data.outflow_year);
    document.getElementById('inflow-year').textContent = formatBRL(data.inflow_year);

    // Update table
    const tbody = document.querySelector('#forecastTable tbody');
    tbody.innerHTML = '';

    for (let i = 0; i < data.outflow_inflation.length; i++) {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${i + 1}</td>
            <td>${formatBRL(data.cumulative_outflow[i])}</td>
            <td>${formatBRL(data.cumulative_inflow[i])}</td>
            <td>${formatBRL(data.outflow_inflation[i])}</td>
            <td>${formatBRL(data.cumulative_outflow_inflation[i])}</td>
            <td>${formatBRL(data.net_flow[i])}</td>
        `;
        tbody.appendChild(row);
    }
}

updateSliderValue('slider1');
updateSliderValue('slider2');
updateSliderValue('slider3');
updateSliderValue('slider4');
updateSliderValue('slider5');

requestForecast('btn')