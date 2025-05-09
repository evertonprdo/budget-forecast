function updateSliderValue(sliderId) {
    const slider = document.getElementById(sliderId);
    const output = document.getElementById(sliderId + '-value');
    output.textContent = slider.value;
    
    slider.oninput = function() {
        output.textContent = this.value;
    }
}

updateSliderValue('slider1');
updateSliderValue('slider2');
updateSliderValue('slider3');