export default function Footer() {
    return (
        <footer className=" fixed bottom-0 left-0 right-0 h-10 group z-40">
            <div className="absolute inset-0"> </div>
            <div className="absolute inset-0 flex items-center justify-between px-4 opacity-0 transition-all duration-200 group-hover:opacity-200">
                <p className="text-white">This is footer</p>
            </div>
        </footer>
    )
}