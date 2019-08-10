	
        var tracks = [];
        var track_struct;
        var url = '/';
        var i = 0;
        var timeDelay = 0;
        const getUrl = "/get_songs";
        const stopUrl = "/stop";
        const playUrl = "/play";

        var song_list = $("#avail_songs");

	////////////////////////////////////////////////
	//post_songs
	// 	This function takes a song title and 
	// 	sends it to the back end rust program to 
	// 	be added to the list of songs to play 
	//	Parameters: 
	// 		song_title: This is the title 
	//                          the song you wish
	//			    to hear. 
	// Function Author: Paul Hubbard
	////////////////////////////////////////////////
        function post_song(song_title) {
            url += song_title;
            $.post(url, function(track_struct, status) {
                console.log(`${track_struct} and status is ${status}`)
            });

            url = '/';
        }

	///////////////////////////////////////////////
	//empty_track_list:
	// 	This clears the tracks array, that holds 
	// 	all of the Track structs that you wish
	// 	to hear. 
	// Function Author: Paul Hubbard 
	///////////////////////////////////////////////
        function empty_track_list() {
            tracks = [];
        }

	//////////////////////////////////////////////
	//my_loop
	//	This function is used to load the appropriate
	//	album info while the tracks are being 
	// 	played. 
	// 	Parameters: 
	//		time: This is the amount of time
	//		      in seconds, which then 
	// 		      gets converted to milliseconds,
	//		      that you need the album info 
	// 		      to stay on the screen. 
	// Function Author: Paul Hubbard
	//////////////////////////////////////////////
        function my_loop(time) {
            setTimeout(function() {
                $("#song_info").text(tracks[i].title + ' by ' + tracks[i].artist);
		new_cover = tracks[i].cover.replace("static/", "");
		console.log(new_cover);
		$("#album_art").attr("src", new_cover);
                console.log(tracks[i]);
                i++;
                if (i < tracks.length) {
                    timeDelay = tracks[i].duration.secs * 1000;
                    my_loop(timeDelay);
                }
		else{
			//Send the stop at the end
			//to clear the playlist 
			//of any extra garbage
			$.post(stopUrl, function(status) {
			    console.log(`status is ${status}`)
			});
		}

            }, time)
        }

	////////////////////////////////////////////////
	//postTracks:
	//	This posts the list of Track structs
	// 	that you wish to play as a JSON object
	//	to the back end for processing.
	// Function Author: Paul Hubbard
	///////////////////////////////////////////////
        function postTracks() {
            $.ajax({
                url: '/load_songs',
                type: 'POST',
                data: JSON.stringify(track_struct),
                dataType: 'json',
                contentType: 'application/json'
            });
        }

        function get_tracks() {
            empty_track_list();
            song_list.empty();
            $.getJSON(getUrl, function(result) {

                $('.list-group-item').remove();
                track_struct = result;
                tracks = result.track_list;
                $.each(tracks, function(i) {
                    console.log(result);
                    var li = $('<li/>')
                        .addClass("list-group-item")
                        .attr('role', 'groupitme')
                        .text(tracks[i].title)
                        .appendTo(song_list);
                    var span = $('<span/>')
                        .addClass('badge')
                        .text(' by ' + tracks[i].artist)
                        .appendTo(li);
                });
            });
        }

       
