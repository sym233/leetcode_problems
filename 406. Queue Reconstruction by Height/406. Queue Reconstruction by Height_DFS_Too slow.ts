/**
 * @param {number[][]} people
 * @return {number[][]}
 */
type aPeople = number[];
type PeopleIndex = number;
type QueueIndex = number[];
type Que = aPeople[];

const reconstructQueue = function(people: Que): Que {
    const length = people.length;

    if(length === 0){
        return [];
    }
    function queueCanAddPeople(queue: QueueIndex, newPerson: PeopleIndex): boolean{
        const height = people[newPerson][0];
        const frontHigher = people[newPerson][1];
        let higherThan = 0;
        for(const personI of queue){
            if(people[personI][0] >= height){
                higherThan++;
            }
        }
        return higherThan === frontHigher;
    }

    function tryIt(queues: QueueIndex[]): QueueIndex[]{
        const newQueues = [];
        for(const queue of queues){
            for(let i = 0; i < length; i++){
                if(!queue.includes(i)){
                    if(queueCanAddPeople(queue, i)){
                        newQueues.push(queue.concat(i));
                    }
                }
            }
        }
        console.log(newQueues);
        if(newQueues.length > 0){
            if(newQueues[0].length === length){
                return newQueues;
            }else{
                return tryIt(newQueues);
            }
        }else{
            throw new Error('not possible');
        }
    }
    const res = tryIt([[]]);
    return res[0].map(i=>people[i]);
};
