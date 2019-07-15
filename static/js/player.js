$(document).ready(function () {
            $("#ajaxSubmit").click(function (){
                    var sleepTime =$(this).attr('sleepTime');
                setTimeout(function() {

                                    //$('#progressBarCenter').modal('toggle');
                    console.log('will get printed after 2 seconds')
                }, sleepTime);
            });
 });


<button type="button" class="btn btn-primary" data-toggle="modal" data-target="#progressBarCenter" id="ajaxSubmit" sleepTime=2000 > Execute Script</button>
