scrwidth = workspace.activeScreen.geometry.width;
scrheight = workspace.activeScreen.geometry.height;

Xpos_1p = [0];
Ypos_1p = [0];
Xsize_1p = [scrwidth];
Ysize_1p = [scrheight];

// Vertical split (side by side)
Xpos_2p = [0, scrwidth / 2];
Ypos_2p = [0, 0];
Xsize_2p = [scrwidth / 2, scrwidth / 2];
Ysize_2p = [scrheight, scrheight];

Xpos_3p = [0, 0, scrwidth / 2];
Ypos_3p = [0, scrheight / 2, scrheight / 2];
Xsize_3p = [scrwidth, scrwidth / 2, scrwidth / 2];
Ysize_3p = [scrheight / 2, scrheight / 2, scrheight / 2];

Xpos_4p = [0, scrwidth / 2, 0, scrwidth / 2];
Ypos_4p = [0, 0, scrheight / 2, scrheight / 2];
Xsize_4p = [scrwidth / 2, scrwidth / 2, scrwidth / 2, scrwidth / 2];
Ysize_4p = [scrheight / 2, scrheight / 2, scrheight / 2, scrheight / 2];

function gamescopeSplitscreen(){
    var allClients = workspace.windowList();
    var gamescopeClients = []

    for (var i = 0; i < allClients.length; i++){
        if (allClients[i].resourceClass == 'gamescope'){
            gamescopeClients.push(allClients[i]);
        }
    }
    switch (gamescopeClients.length){
        case 0:
            return;
        case 1:
            var Xpos = Xpos_1p;
            var Ypos = Ypos_1p;
            var Xsize = Xsize_1p;
            var Ysize = Ysize_1p;
            break;
        case 2:
            var Xpos = Xpos_2p;
            var Ypos = Ypos_2p;
            var Xsize = Xsize_2p;
            var Ysize = Ysize_2p;
            break;
        case 3:
            var Xpos = Xpos_3p;
            var Ypos = Ypos_3p;
            var Xsize = Xsize_3p;
            var Ysize = Ysize_3p;
            break;
        case 4:
            var Xpos = Xpos_4p;
            var Ypos = Ypos_4p;
            var Xsize = Xsize_4p;
            var Ysize = Ysize_4p;
            break;
    }

    for (var i = 0; i < gamescopeClients.length; i++){
        gamescopeClients[i].noBorder = true;
        gamescopeClients[i].frameGeometry = {
            x: Xpos[i],
            y: Ypos[i],
            width: Xsize[i],
            height: Ysize[i]
        };
    }
}

workspace.windowAdded.connect(gamescopeSplitscreen);
workspace.windowRemoved.connect(gamescopeSplitscreen);
