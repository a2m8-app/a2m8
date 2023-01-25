/** @jsxImportSource preact */
import type { FunctionalComponent } from "preact";
import { useState, useEffect } from "preact/hooks";

export const ImageCarousel = ({ images }: { images: string[] }) => {
  //   const [recentlyClicked, setRecentlyClicked] = useState(false);
  //   useEffect(() => {
  //     if (recentlyClicked) {
  //       setTimeout(() => {
  //         setRecentlyClicked(false);
  //       }, 1000);
  //     }
  //   }, [recentlyClicked]);

  return (
    <div class="carousel w-[74rem] mx-auto h-[40rem]">
      {images.map((image, index) => {
        return (
          <div id={`slide${index}`} class="carousel-item relative w-full">
            <img
              src={image}
              class="mx-auto object-contain "
              onClick={() => {
                //open in popup
                window.open(
                  image,
                  "test",
                  "crollbars=no,resizable=no,status=no,location=no,toolbar=no,menubar=no,width=1000,height=650,left=100,top=100"
                );
              }}
            />
            <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
              <a
                href={`#slide${
                  index - 1 == -1 ? images.length - 1 : index - 1
                }`}
                class="btn btn-circle"
              >
                ❮
              </a>
              <a
                href={`#slide${index + 1 > images.length - 1 ? 0 : index + 1}`}
                class="btn btn-circle"
              >
                ❯
              </a>
            </div>
          </div>
        );
      })}
    </div>
  );
};
