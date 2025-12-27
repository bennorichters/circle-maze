import init, { generate_maze_svg } from './pkg/circle_maze.js';

let wasmModule = null;
let currentSvg = null;

const elements = {
    circlesInput: document.getElementById('circles'),
    generateBtn: document.getElementById('generate-btn'),
    mazeDisplay: document.getElementById('maze-display'),
    togglePathBtn: document.getElementById('toggle-path-btn'),
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

        elements.mazeDisplay.innerHTML = currentSvg;

        elements.togglePathBtn.style.display = 'inline-block';
    } catch (error) {
        showError(`Error generating maze: ${error.message}`);
        elements.mazeDisplay.innerHTML =
            '<div class="loading">Failed to generate maze. Please try again.</div>';
    } finally {
        elements.generateBtn.disabled = false;
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
