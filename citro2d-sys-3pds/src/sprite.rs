use crate::bindings::*;
use crate::C3D_AngleFromDegrees;
use libm::fabsf;

#[inline]
pub unsafe fn C2D_SpriteFromImage(sprite: *mut C2D_Sprite, image: C2D_Image) {
	(*sprite).image           = image;
	(*sprite).params.pos.x    = 0.0 as f32;
	(*sprite).params.pos.y    = 0.0 as f32;
	(*sprite).params.pos.w    = (*image.subtex).width as f32;
	(*sprite).params.pos.h    = (*image.subtex).height as f32;
	(*sprite).params.center.x = 0.0 as f32;
	(*sprite).params.center.y = 0.0 as f32;
	(*sprite).params.angle    = 0.0 as f32;
	(*sprite).params.depth    = 0.0 as f32;
}

#[inline]
pub unsafe fn C2D_SpriteFromSheet(sprite: *mut C2D_Sprite, sheet: C2D_SpriteSheet , index: usize) {
	C2D_SpriteFromImage(sprite, C2D_SpriteSheetGetImage(sheet, index));
}

#[inline]
pub unsafe fn C2D_SpriteScale(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	(*sprite).params.pos.w *= x;
	(*sprite).params.pos.h *= y;
	(*sprite).params.center.x *= x;
	(*sprite).params.center.y *= y;
}

#[inline]
pub unsafe fn C2D_SpriteRotate(sprite: *mut C2D_Sprite, radians: f32) {
	(*sprite).params.angle += radians;
}

#[inline]
pub unsafe fn C2D_SpriteRotateDegrees(sprite: *mut C2D_Sprite, degrees: f32) {
	C2D_SpriteRotate(sprite, C3D_AngleFromDegrees(degrees));
}

#[inline]
pub unsafe fn C2D_SpriteMove(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	(*sprite).params.pos.x += x;
	(*sprite).params.pos.y += y;
}

#[inline]
pub unsafe fn C2D_SpriteSetScale(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	let oldCenterX: f32 = (*sprite).params.center.x / (*sprite).params.pos.w;
	let oldCenterY: f32 = (*sprite).params.center.y / (*sprite).params.pos.h;
	(*sprite).params.pos.w = x*(*(*sprite).image.subtex).width as f32;
	(*sprite).params.pos.h = y*(*(*sprite).image.subtex).height as f32;
	(*sprite).params.center.x = fabsf(oldCenterX*(*sprite).params.pos.w);
	(*sprite).params.center.y = fabsf(oldCenterY*(*sprite).params.pos.h);
}

#[inline]
pub unsafe fn C2D_SpriteSetRotation(sprite: *mut C2D_Sprite, radians: f32) {
	(*sprite).params.angle = radians;
}

#[inline]
pub unsafe fn C2D_SpriteSetRotationDegrees(sprite: *mut C2D_Sprite, degrees: f32) {
	C2D_SpriteSetRotation(sprite, C3D_AngleFromDegrees(degrees));
}

#[inline]
pub unsafe fn C2D_SpriteSetCenter(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	(*sprite).params.center.x = x*(*sprite).params.pos.w;
	(*sprite).params.center.y = y*(*sprite).params.pos.h;
}

#[inline]
pub unsafe fn C2D_SpriteSetCenterRaw(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	(*sprite).params.center.x = x;
	(*sprite).params.center.y = y;
}

#[inline]
pub unsafe fn C2D_SpriteSetPos(sprite: *mut C2D_Sprite, x: f32, y: f32) {
	(*sprite).params.pos.x = x;
	(*sprite).params.pos.y = y;
}

#[inline]
pub unsafe fn C2D_SpriteSetDepth(sprite: *mut C2D_Sprite, depth: f32) {
	(*sprite).params.depth = depth;
}

#[inline]
pub unsafe fn C2D_DrawSprite(sprite: *const C2D_Sprite) -> bool {
	return C2D_DrawImage((*sprite).image, &(*sprite).params, core::ptr::null());
}

#[inline]
pub unsafe fn C2D_DrawSpriteTinted(sprite: *const C2D_Sprite, tint: *const C2D_ImageTint) -> bool
{
	return C2D_DrawImage((*sprite).image, &(*sprite).params, tint);
}