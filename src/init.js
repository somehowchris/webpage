export const hasWebPSupport = () => {
    return new Promise((resolve, reject) => {
      var img = new Image();
      img.onload = () => { resolve(); };
      img.onerror = () => { reject(); };
      img.src = 'data:image/webp;base64,UklGRh4AAABXRUJQVlA4TBEAAAAvAQAAAAfQ//73v/+BiOh/AAA=';
    });
}