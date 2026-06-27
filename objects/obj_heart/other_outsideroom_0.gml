if obj_battlecontroller.runaway=1 then
   {
   if instance_exists(obj_unfader)=false then instance_create(0,0,obj_unfader)
   if x<-60
      {
   caster_stop(global.batmusic)
   caster_free(global.batmusic)
   room_goto(global.currentroom)
      }
   }
