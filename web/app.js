import init, { generate_maze_svg, generate_maze_json } from './pkg/circle_maze.js';

let wasmModule = null;
let currentSvg = null;
let currentJson = null;

const elements = {
    circlesInput: document.getElementById('circles'),
    generateBtn: document.getElementById('generate-btn'),
    mazeDisplay: document.getElementById('maze-display'),
    togglePathBtn: document.getElementById('toggle-path-btn'),
    downloadSvgBtn: document.getElementById('download-svg-btn'),
    downloadJsonBtn: document.getElementById('download-json-btn'),
    errorContainer: document.getElementById('error-container')
};

function showError(message) {
    elements.errorContainer.innerHTML = `<div class="error">${message}</div>`;
    setTimeout(() => {
        elements.errorContainer.innerHTML = '';
    }, 5000);
}

function clearError() {
    elements.errorContainer.innerHTML = '';
}

function downloadFile(content, filename, mimeType) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
}

async function generateMaze() {
    if (!wasmModule) {
        showError('WASM module not loaded yet. Please wait...');
        return;
    }

    clearError();
    const circles = parseInt(elements.circlesInput.value);

    if (circles < 3 || circles > 20) {
        showError('Please enter a number between 3 and 20');
        return;
    }

    elements.generateBtn.disabled = true;
    elements.mazeDisplay.innerHTML = '<div class="loading">Generating maze...</div>';

    try {
        currentSvg = generate_maze_svg(circles);
        currentJson = generate_maze_json(circles);

        elements.mazeDisplay.innerHTML = currentSvg;

        elements.togglePathBtn.style.display = 'inline-block';
        elements.downloadSvgBtn.style.display = 'inline-block';
        elements.downloadJsonBtn.style.display = 'inline-block';
    } catch (error) {
        showError(`Error generating maze: ${error.message}`);
        elements.mazeDisplay.innerHTML =
            '<div class="loading">Failed to generate maze. Please try again.</div>';
    } finally {
        elements.generateBtn.disabled = false;
    }
}

function downloadSvg() {
    if (currentSvg) {
        const circles = elements.circlesInput.value;
        downloadFile(currentSvg, `circle-maze-${circles}.svg`, 'image/svg+xml');
    }
}

function downloadJson() {
    if (currentJson) {
        const circles = elements.circlesInput.value;
        downloadFile(currentJson, `circle-maze-${circles}.json`, 'application/json');
    }
}

function togglePath() {
    const path = document.querySelector('#solution-path');
    if (path) {
        path.classList.toggle('visible');
        elements.togglePathBtn.textContent = path.classList.contains('visible')
            ? 'Hide Path'
            : 'Show Path';
    }
}

async function initApp() {
    try {
        elements.mazeDisplay.innerHTML = '<div class="loading">Loading WebAssembly module...</div>';

        wasmModule = await init();

        elements.mazeDisplay.innerHTML =
            '<div class="loading">Click "Generate Maze" to create a circular maze</div>';

        elements.generateBtn.addEventListener('click', generateMaze);
        elements.togglePathBtn.addEventListener('click', togglePath);
        elements.downloadSvgBtn.addEventListener('click', downloadSvg);
        elements.downloadJsonBtn.addEventListener('click', downloadJson);

        elements.circlesInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                generateMaze();
            }
        });
    } catch (error) {
        showError(`Failed to initialize WebAssembly: ${error.message}`);
        elements.mazeDisplay.innerHTML =
            '<div class="loading">Failed to load. Please refresh the page.</div>';
    }
}

initApp();
