if instance_exists(obj_torface) then
  {
  myinteract=1
  if OBJ_WRITER.halt != 0 then image_speed=0.2
  } 
 else
     {
    myinteract=0
    if speed=0 then 
       {
       image_index=0
       image_speed=0
       }
  if speed>0 then image_speed=0.20
     }  
  
script_execute(scr_npcdir)
