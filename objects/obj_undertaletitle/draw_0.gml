draw_sprite_ext(sprite_index,image_index,x,y,1,1,rot,c_white,1)

if ok=1
{
draw_set_color(c_gray)
draw_set_font(fnt_small)
if jok=0
  {
  draw_text(30,150,"Written, programmed, and composed by Toby Fox")
  draw_text(30,162,"Black and white art by Toby Fox")
  draw_text(30,174,"Intro art and overworld characters by Tuyoki")
  draw_text(30,186,"Tiles by Tuyoki, easynam, and Merrigo")
  draw_text(30,206,"Caster OGG extension by Moacube")
  draw_text(30,216,"and Marius Utheim")
  }
else
  {
  draw_set_font(fnt_maintext)
  draw_set_color(c_red)
   draw_text(30,150,"That was fun.")
   draw_text(30,170,"Let's finish the job.")
   }

}

