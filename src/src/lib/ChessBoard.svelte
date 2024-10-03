
<script>
  import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte';
  let fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
  let board = '';
  let boardArray = board.split('');
  onMount(() => {
        render();
  });
  async function render() {
    board = await invoke('render', { fen });
    boardArray = board.split('');
    const pieceMap = {
    r: '♜', // black rook
    n: '♞', // black knight
    b: '♝', // black bishop
    q: '♛', // black queen
    k: '♚', // black king
    p: '♟', // black pawn
    R: '♖', // white rook
    N: '♘', // white knight
    B: '♗', // white bishop
    Q: '♕', // white queen
    K: '♔', // white king
    P: '♙', // white pawn
    ' ': ' '  // empty square
  };
  const renderedBoard = boardArray.map(piece => pieceMap[piece] || piece).join('');
  boardArray = renderedBoard.split('');
  }
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
        </style>
          <input id="greet-input" placeholder="Enter a name..." bind:value="{fen}" />
          <button on:click="{render}">Enter FEN</button>
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
                    <td class="light">{boardArray[0]}</td>
                    <td class="dark">{boardArray[1]}</td>
                    <td class="light">{boardArray[2]}</td>
                    <td class="dark">{boardArray[3]}</td>
                    <td class="light">{boardArray[4]}</td>
                    <td class="dark">{boardArray[5]}</td>
                    <td class="light">{boardArray[6]}</td>
                    <td class="dark">{boardArray[7]}</td>
                </tr>
                <tr>
                    <th>7</th>
                    <td class="dark">{boardArray[8]}</td>
                    <td class="light">{boardArray[9]}</td>
                    <td class="dark">{boardArray[10]}</td>
                    <td class="light">{boardArray[11]}</td>
                    <td class="dark">{boardArray[12]}</td>
                    <td class="light">{boardArray[13]}</td>
                    <td class="dark">{boardArray[14]}</td>
                    <td class="light">{boardArray[15]}</td>
                </tr>
                <tr>
                    <th>6</th>
                    <td class="light">{boardArray[16]}</td>
                    <td class="dark">{boardArray[17]}</td>
                    <td class="light">{boardArray[18]}</td>
                    <td class="dark">{boardArray[19]}</td>
                    <td class="light">{boardArray[20]}</td>
                    <td class="dark">{boardArray[21]}</td>
                    <td class="light">{boardArray[22]}</td>
                    <td class="dark">{boardArray[23]}</td>
                </tr>
                <tr>
                    <th>5</th>
                    <td class="dark">{boardArray[24]}</td>
                    <td class="light">{boardArray[25]}</td>
                    <td class="dark">{boardArray[26]}</td>
                    <td class="light">{boardArray[27]}</td>
                    <td class="dark">{boardArray[28]}</td>
                    <td class="light">{boardArray[29]}</td>
                    <td class="dark">{boardArray[30]}</td>
                    <td class="light">{boardArray[31]}</td>
                </tr>
                <tr>
                    <th>4</th>
                    <td class="light">{boardArray[32]}</td>
                    <td class="dark">{boardArray[33]}</td>
                    <td class="light">{boardArray[34]}</td>
                    <td class="dark">{boardArray[35]}</td>
                    <td class="light">{boardArray[36]}</td>
                    <td class="dark">{boardArray[37]}</td>
                    <td class="light">{boardArray[38]}</td>
                    <td class="dark">{boardArray[39]}</td>
                </tr>
                <tr>
                    <th>3</th>
                    <td class="dark">{boardArray[40]}</td>
                    <td class="light">{boardArray[41]}</td>
                    <td class="dark">{boardArray[42]}</td>
                    <td class="light">{boardArray[43]}</td>
                    <td class="dark">{boardArray[44]}</td>
                    <td class="light">{boardArray[45]}</td>
                    <td class="dark">{boardArray[46]}</td>
                    <td class="light">{boardArray[47]}</td>
                </tr>
                <tr>
                    <th>2</th>
                    <td class="light">{boardArray[48]}</td>
                    <td class="dark">{boardArray[49]}</td>
                    <td class="light">{boardArray[50]}</td>
                    <td class="dark">{boardArray[51]}</td>
                    <td class="light">{boardArray[52]}</td>
                    <td class="dark">{boardArray[53]}</td>
                    <td class="light">{boardArray[54]}</td>
                    <td class="dark">{boardArray[55]}</td>
                </tr>
                <tr>
                    <th>1</th>
                    <td class="dark">{boardArray[56]}</td>
                    <td class="light">{boardArray[57]}</td>
                    <td class="dark">{boardArray[58]}</td>
                    <td class="light">{boardArray[59]}</td>
                    <td class="dark">{boardArray[60]}</td>
                    <td class="light">{boardArray[61]}</td>
                    <td class="dark">{boardArray[62]}</td>
                    <td class="light">{boardArray[63]}</td>
                </tr>
            </tbody>
        </table>