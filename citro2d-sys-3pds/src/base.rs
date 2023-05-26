use crate::bindings::*;
use citro3d_sys::{C3D_FrameDrawOn, M_TAU};

pub fn C3D_AngleFromDegrees(_angle: f32) -> f32 {(_angle)*M_TAU as f32/360.0 as f32}

pub fn C2D_Clamp(x: f32, min: f32, max: f32) -> f32 {
    if x <= min {
        return min;
    }
    else if x >= max {
        return max;
    }
    x
}

pub fn C2D_FloatToU8(x: f32) -> u8 {
	(255.0 as f32*C2D_Clamp(x, 0.0 as f32, 1.0 as f32)+0.5 as f32) as u8
}

#[allow(arithmetic_overflow)]
pub fn C2D_Color32(r: u8, g: u8, b: u8, a: u8) -> u32 {
	return (r | (g << (8 as u32)) | (b << (16 as u32)) | (a << (24 as u32))) as u32;
}

pub fn C2D_Color32f(r: f32, g: f32, b: f32, a: f32) -> u32
{
	return C2D_Color32(C2D_FloatToU8(r),C2D_FloatToU8(g),C2D_FloatToU8(b),C2D_FloatToU8(a));
}

#[inline]
pub unsafe fn C2D_SetImageTint(tint: *mut C2D_ImageTint, corner: C2D_Corner, color: u32, blend: f32) {
    (*tint).corners[corner as usize].color = color;
    (*tint).corners[corner as usize].blend = blend;
}

#[inline]
pub unsafe fn C2D_PlainImageTint(tint: *mut C2D_ImageTint, color: u32, blend: f32) {
	C2D_SetImageTint(tint, C2D_TopLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_TopRight, color, blend);
	C2D_SetImageTint(tint, C2D_BotLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_AlphaImageTint(tint: *mut C2D_ImageTint, alpha: f32) {
	C2D_PlainImageTint(tint, C2D_Color32f(0.0 as f32, 0.0 as f32, 0.0 as f32, alpha), 0.0 as f32);
}

#[inline]
pub unsafe fn C2D_TopImageTint(tint: *mut C2D_ImageTint, color: u32, blend: f32) {
	C2D_SetImageTint(tint, C2D_TopLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_TopRight, color, blend);
}

#[inline]
pub unsafe fn C2D_BottomImageTint(tint: *mut C2D_ImageTint, color: u32, blend: f32) {
	C2D_SetImageTint(tint, C2D_BotLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_LeftImageTint(tint: *mut C2D_ImageTint, color: u32, blend: f32) {
	C2D_SetImageTint(tint, C2D_TopLeft, color, blend);
	C2D_SetImageTint(tint, C2D_BotLeft, color, blend);
}

#[inline]
pub unsafe fn C2D_RightImageTint(tint: *mut C2D_ImageTint, color: u32, blend: f32) {
	C2D_SetImageTint(tint, C2D_TopRight, color, blend);
	C2D_SetImageTint(tint, C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_SceneTarget(target: *mut C3D_RenderTarget)
{
	C2D_SceneSize((*target).frameBuf.width as u32, (*target).frameBuf.height as u32, (*target).linked);
}

#[inline]
pub unsafe fn C2D_ViewRotateDegrees(rotation: u32)
{
	C2D_ViewRotate(C3D_AngleFromDegrees(rotation as f32));
}

#[inline]
pub unsafe fn C2D_SceneBegin(target: *mut C3D_RenderTarget)
{
	C2D_Flush();
	C3D_FrameDrawOn(target as *mut citro3d_sys::C3D_RenderTarget_tag);
	C2D_SceneTarget(target);
}


#[inline]
pub unsafe fn C2D_DrawImageAt(img: C2D_Image, x: f32, y: f32, depth: f32,
    tint: Option<*const C2D_ImageTint>,
	scaleX: Option<f32>, scaleY: Option<f32>) -> bool
{
    let t_tint: *const C2D_ImageTint;
    if let Some(t) = tint {
        t_tint = t;
    }
    else {
        t_tint = core::ptr::null();
    }
    let t_scaleX: f32;
    if let Some(t) = scaleX {
        t_scaleX = t;
    }
    else {
        t_scaleX = 1.0 as f32;
    }
    let t_scaleY: f32;
    if let Some(t) = scaleY {
        t_scaleY = t;
    }
    else {
        t_scaleY = 1.0 as f32;
    }
    let params: C2D_DrawParams = C2D_DrawParams {
        pos: C2D_DrawParams__bindgen_ty_1{x, y, w: t_scaleX * (*img.subtex).width as f32, h: t_scaleY * (*img.subtex).height as f32}, 
        center: C2D_DrawParams__bindgen_ty_2 {x: 0.0 as f32, y: 0.0 as f32}, 
        depth, angle: 0.0 as f32
    };
	return C2D_DrawImage(img, &params, t_tint);
}

#[inline]
pub unsafe fn C2D_DrawImageAtRotated(img: C2D_Image, x: f32, y: f32, depth: f32, angle: f32,
    tint: Option<*const C2D_ImageTint>,
	scaleX: Option<f32>, scaleY: Option<f32>) -> bool
{
    let t_tint: *const C2D_ImageTint;
    if let Some(t) = tint {
        t_tint = t;
    }
    else {
        t_tint = core::ptr::null();
    }
    let t_scaleX: f32;
    if let Some(t) = scaleX {
        t_scaleX = t;
    }
    else {
        t_scaleX = 1.0 as f32;
    }
    let t_scaleY: f32;
    if let Some(t) = scaleY {
        t_scaleY = t;
    }
    else {
        t_scaleY = 1.0 as f32;
    }
    let params: C2D_DrawParams = C2D_DrawParams {
        pos: C2D_DrawParams__bindgen_ty_1{x, y, w: t_scaleX * (*img.subtex).width as f32, h: t_scaleY * (*img.subtex).height as f32}, 
        center: C2D_DrawParams__bindgen_ty_2 {x: (t_scaleX * (*img.subtex).width as f32)/2.0 as f32, y: (t_scaleY * (*img.subtex).height as f32) as f32}, 
        depth, angle
    };
	return C2D_DrawImage(img, &params, t_tint);
}

#[inline]
pub unsafe fn C2D_DrawRectSolid(
	x: f32, y: f32, z: f32, w: f32, h: f32,
	clr: u32) -> bool
{
	return C2D_DrawRectangle(x,y,z,w,h,clr,clr,clr,clr);
}

#[inline]
pub unsafe fn C2D_DrawEllipseSolid(
	x: f32, y: f32, z: f32, w: f32, h: f32,
	clr: u32) -> bool
{
	return C2D_DrawEllipse(x,y,z,w,h,clr,clr,clr,clr);
}

#[inline]
pub unsafe fn C2D_DrawCircle(
	x: f32, y: f32, z: f32, radius: f32,
	clr0: u32, clr1: u32, clr2: u32, clr3: u32) -> bool
{
	return C2D_DrawEllipse(
		x - radius,y - radius,z,radius*2.0,radius*2.0,
		clr0,clr1,clr2,clr3);
}

#[inline]
pub unsafe fn C2D_DrawCircleSolid(
	x: f32, y: f32, z: f32, radius: f32,
	clr: u32) -> bool
{
	return C2D_DrawCircle(x,y,z,radius,clr,clr,clr,clr);
}