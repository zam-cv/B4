mergeInto(LibraryManager.library, {
  GetHostname: function () {
    var result = window.location.hostname;
    var bufferSize = lengthBytesUTF8(result) + 1;
    var buffer = _malloc(bufferSize);
    stringToUTF8(result, buffer, bufferSize);
    return buffer;
  }
});
