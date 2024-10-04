<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { Chessground } from 'svelte-chessground-ui';

	
	let fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
	let board = '';
	let boardArray = board.split('');
	onMount(() => {
		render();
	});
	async function render() {
		board = await invoke('render', { fen });
	}
</script>

<input id="greet-input" placeholder="Enter a name..." bind:value={fen} />
<Chessground fen={fen}/>