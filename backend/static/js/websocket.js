$(function() {

	var conn = null;
	function clog(msg) {
		var control = $('#clientlog');
		control.html(control.html() + msg + '<br/>');
		control.scrollTop(control.scrollTop() + 1000);
	}
	function log(msg) {
		var control = $('#serverlog');
		control.html(control.html() + msg + '<br/>');
		control.scrollTop(control.scrollTop() + 1000);
	}
	function connect() {
		disconnect();
		var wsUri = (window.location.protocol == 'https:' && 'wss://' || 'ws://')
				+ window.location.host + '/router/';
		conn = new WebSocket(wsUri);
		clog('Connecting...');
		conn.onopen = function() {
			clog('Connected.');
			update_ui();
		};
		conn.onmessage = function(e) {
			clog('Received: ' + e.data);
		};
		conn.onclose = function() {
			clog('Disconnected.');
			conn = null;
			update_ui();
		};
	}
	function disconnect() {
		if (conn != null) {
			clog('Disconnecting...');
			conn.close();
			conn = null;
			update_ui();
		}
	}
	function send(head, body) {
		var msg = {
			type : "msg",
			head : head,
			body : body
		};
		if (conn != null) {
			conn.send(JSON.stringify(msg));
		}
		else {
			alert("Error: could not send! not connected !");
		}
	}
	function keygen() {
		var head = {
				type : "keygen",
				date : Date.now()
			};		
		send(head, "");
	}
	function upload(d) {
		var head = {
				type : "upload",
				date : Date.now()
			};
		var body = {
				data : d
			};		
		send(head, body);
	}
	function update_ui() {
		var msg = '';
		if (conn == null) {
			$('#status').text('disconnected');
			$('#connect').html('Connect');
		} else {
			$('#status').text('connected (' + conn.protocol + ')');
			$('#connect').html('Disconnect');
		}
	}
	$('#connect').click(function() {
		if (conn == null) {
			connect();
		} else {
			disconnect();
		}
		update_ui();
		return false;
	});
	$('#key').click(function() {
		if (conn == null) {
			connect();
		}
		keygen();
		update_ui();
		return false;
	});
	$('#send').click(function() {
		var text = $('#text').val();
		log('Sending: ' + text);
		conn.send(text);
		$('#text').val('').focus();
		return false;
	});
	$('#text').keyup(function(e) {
		if (e.keyCode === 13) {
			$('#send').click();
			return false;
		}
	});
});