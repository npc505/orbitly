import React from "react";

interface Props {
  children: React.ReactNode;
}

export default function MobileFrame({ children }: Props) {
  return (
    <div className="w-full min-h-screen bg-gray-200 flex justify-center items-center">
      <div className="relative w-[390px] h-[844px] bg-gray-800 rounded-[40px] shadow-2xl overflow-hidden">

        {/* 🔥 Notch arriba */}
        <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[180px] h-[35px] bg-black rounded-b-3xl z-20"></div>

        {/* 🔥 Contenido movido hacia abajo para no tapar el notch */}
        <div className="pt-[50px] h-full overflow-y-auto">
          {children}
        </div>

      </div>
    </div>
  );
}
