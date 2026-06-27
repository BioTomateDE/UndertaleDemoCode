global.entrance=24 // 1 = forward, 2=backward, 3=other, 0=custom
instance_create(0,0,obj_persistentfader)
if room=33 then room_goto(38)
if room=38 then room_goto(33)
if room=room_ruins15E then room_goto(room_ruins15A)
if room=room_ruins16 then room_goto(room_ruins19)
if room=room_ruins19 then room_goto(room_ruins16)
if room=room_torhouse1 then {caster_free(global.currentsong2); room_goto(room_basement1)}
if room=room_basement1 then room_goto(room_torhouse1)
if room=room_torhouse2 then room_goto(room_kitchen)
if room=room_kitchen then room_goto(room_torhouse2)
