(function() {
    const evtSource = new EventSource('/__reload__');
    evtSource.onmessage = function(e) {
        if (e.data === 'reload') {
            console.log('File changed, reloading...');
            location.reload();
        }
    };
    evtSource.onerror = function(e) {
        // EventSourceは自動的に再接続を試みるので、ここでリロードする必要はない
        console.log('SSE connection error, will auto-retry...', e);
    };
})();
