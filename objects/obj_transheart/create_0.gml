xx=view_xview[view_current]
yy=view_yview[view_current]

mychoicex=xx+20
mychoicey=yy+223

if room=room_area1_2 then
  {
  mychoicex=xx+154
  mychoicey=yy+156
  }

spdr=distance_to_point(mychoicex,mychoicey)/17
move_towards_point(mychoicex,mychoicey,spdr)
sound_play(snd_battlefall)
