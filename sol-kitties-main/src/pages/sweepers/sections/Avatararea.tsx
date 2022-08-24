import React from 'react'

const Avatararea = () => {
  return (
    <div className='min-h-[100px]'>
      <div className='main-layout avatararea relative'>
        <div className='main-container flex z-50 flex-wrap justify-center md:justify-start'>
          <div className='px-10'>
            <img src='images/kitty-footer.png' alt='kitty-footer.png' className='w-[380px] h-full'/>
          </div>
          <div className='items-end flex flex-col justify-end pb-8'>
            <p className='uppercase text-[#50fc42] text-[10.65px] font-[Orbitron]'>POWERED BY</p>
            <img src='images/solana.png' alt='download' className='w-[158px] h-[30px] mt-2'/>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Avatararea