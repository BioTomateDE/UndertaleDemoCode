//script_execute(SCR_DIRECT, 3, 1, 10, 0.5, 270, 0.1, 90, 6, 8, spr_dirbullet)
//script_execute(SCR_DIRECT, 0, 0, 60, 0.4, 270, 0, 90, 6, 18, spr_dirbullet)
r=round(random(1))+2
script_execute(SCR_BORDER,r,2)
x = xx
y = yy
if bullettype=0 then
{
iii=instance_create(x,y,blt_hoopbullet1)
iii.dmg= global.monsteratk[myself]
iii.bullettype=0
}
else
   {
   r=round(random(3))
   script_execute(SCR_BORDER,r,8)
   iii=instance_create(x,y,blt_hoopbullet2)
   iii.dmg= global.monsteratk[myself]
   }

   
alarm[0]=firingspeed
