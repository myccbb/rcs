export default function Separator({ orientation, className }: {
    orientation: 'horizontal' | 'vertical',
    className?: string,
}) {
    if (orientation == "vertical") {
        return <div
            className={'w-0.5 border-none h-full bg-gray-300 ' + className ?? ''}
        />
    }
    return <hr
        className={'h-0.5 border-none w-full bg-gray-300 ' + className ?? ''}
    />
}