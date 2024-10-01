
<script>
  import { invoke } from '@tauri-apps/api/tauri'

  let fen = '';
  let board = '';

  async function render() {
    board = await invoke('render', { fen });
  }
    let draggedPiece = null;

    document.querySelectorAll('.chess-board td').forEach(square => {
        // Drag start event: store the dragged piece
        square.addEventListener('dragstart', (event) => {
            draggedPiece = event.target;
        });

        // Drag over event: allow dropping by preventing the default
        square.addEventListener('dragover', (event) => {
            event.preventDefault();
            square.classList.add('drag-over');
        });

        // Drag leave event: remove the drag-over class
        square.addEventListener('dragleave', () => {
            square.classList.remove('drag-over');
        });

        // Drop event: handle dropping the piece
        square.addEventListener('drop', (event) => {
            event.preventDefault();
            square.classList.remove('drag-over');

            // Check if the target is empty before dropping
            if (!square.textContent) {
                square.textContent = draggedPiece.textContent;
                draggedPiece.textContent = ''; // Remove the piece from the original square
            }
        });

        // Drag end event: clean up
        square.addEventListener('dragend', () => {
            draggedPiece = null;
        });
    });
</script>


        <style>
            .chess-board { border-spacing: 0; border-collapse: collapse; }
            .chess-board th { padding: .5em; }
            .chess-board th + th { border-bottom: 1px solid #000; }
            .chess-board th:first-child,
            .chess-board td:last-child { border-right: 1px solid #000; }
            .chess-board tr:last-child td { border-bottom: 1px solid; }
            .chess-board th:empty { border: none; }
            .chess-board td { width: 1.5em; height: 1.5em; text-align: center; font-size: 32px; line-height: 0;}
            .chess-board .light { background: #eee; }
            .chess-board .dark { background: #aaa; }

            .chess-board td {
    width: 50px;
    height: 50px;
    text-align: center;
    font-size: 2em;
}

.chess-board td.drag-over {
    background-color: yellow; /* Highlight drop target */
}
        </style>
          <input id="greet-input" placeholder="Enter a name..." bind:value="{fen}" />
          <button on:click="{render}">Enter FEN</button>
          <p>{board}</p>
        <table class="chess-board">
            <tbody>
                <tr>
                    <th></th>
                    <th>a</th>
                    <th>b</th>
                    <th>c</th>
                    <th>d</th>
                    <th>e</th>
                    <th>f</th>
                    <th>g</th>
                    <th>h</th>
                </tr>
                <tr>
                    <th>8</th>
                    <td class="light" draggable="true">♜</td>
                    <td class="dark">♞</td>
                    <td class="light">♝</td>
                    <td class="dark">♛</td>
                    <td class="light">♚</td>
                    <td class="dark">♝</td>
                    <td class="light">♞</td>
                    <td class="dark">♜</td>
                </tr>
                <tr>
                    <th>7</th>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                </tr>
                <tr>
                    <th>6</th>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                </tr>
                <tr>
                    <th>5</th>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                </tr>
                <tr>
                    <th>4</th>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                </tr>
                <tr>
                    <th>3</th>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                </tr>
                <tr>
                    <th>2</th>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                </tr>
                <tr>
                    <th>1</th>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                    <td class="dark"></td>
                    <td class="light"></td>
                </tr>
            </tbody>
        </table>